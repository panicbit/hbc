use std::io::BufRead;

use anyhow::{Context, Result};
use num_traits::FromPrimitive;

use super::{ValueType, OBJECT_END, STRING_END};

pub struct TokenDeserializer<R> {
    reader: R,
    next_op: NextOp,
    depth: usize,
}

impl<R> TokenDeserializer<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            next_op: NextOp::ParseValue(ValueType::Object),
            depth: 0,
        }
    }
}

impl<R> Iterator for TokenDeserializer<R>
where
    R: BufRead,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.next_op {
            NextOp::ParseValue(value_type) => self.handle_parse_value(value_type),
            NextOp::ParseKey => self.handle_parse_key(),
            NextOp::DoNothing => return None,
        };

        match result {
            Ok((token, next_op)) => {
                self.next_op = next_op;
                Some(Ok(token))
            }
            Err(err) => {
                self.next_op = NextOp::DoNothing;
                Some(Err(err))
            }
        }
    }
}

impl<R> TokenDeserializer<R>
where
    R: BufRead,
{
    fn handle_parse_key(&mut self) -> Result<(Token, NextOp)> {
        let value_type = self.read_value_type()?;
        let value_type = match value_type {
            Some(value_type) => value_type,
            None => {
                self.depth -= 1;

                let next_op = if self.depth == 0 {
                    NextOp::DoNothing
                } else {
                    NextOp::ParseKey
                };

                return Ok((Token::ObjectEnd, next_op));
            }
        };

        let key = self.read_string()?;

        Ok((Token::Key(key), NextOp::ParseValue(value_type)))
    }

    fn handle_parse_value(&mut self, value_type: ValueType) -> Result<(Token, NextOp)> {
        let token = match value_type {
            ValueType::Object => {
                self.depth += 1;
                Token::ObjectStart
            }
            ValueType::String => {
                let value = self.read_string()?;

                Token::String(value)
            }
            ValueType::Int => {
                let value = self.read_int()?;

                Token::Int(value)
            }
        };

        Ok((token, NextOp::ParseKey))
    }

    fn read_value_type(&mut self) -> Result<Option<ValueType>> {
        let byte = self.read_byte()?;

        if byte == OBJECT_END {
            return Ok(None);
        }

        let value_type = ValueType::from_u8(byte)
            .with_context(|| format!("unknown value type: 0x{byte:02x}"))?;

        Ok(Some(value_type))
    }

    fn read_string(&mut self) -> Result<String> {
        let mut string = Vec::new();

        self.reader.read_until(STRING_END, &mut string)?;

        string.pop();

        let string = String::from_utf8(string)?;

        Ok(string)
    }

    fn read_int(&mut self) -> Result<u32> {
        let mut buf = [0; 4];

        self.reader.read_exact(&mut buf)?;

        let n = u32::from_le_bytes(buf);

        Ok(n)
    }

    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0];

        self.reader.read_exact(&mut buf)?;

        Ok(buf[0])
    }
}

#[derive(Copy, Clone)]
enum NextOp {
    DoNothing,
    ParseValue(ValueType),
    ParseKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    ObjectStart,
    ObjectEnd,
    Key(String),
    Int(u32),
    String(String),
}
