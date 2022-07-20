use std::io::Write;

use serde::{de, ser};

use super::{ValueType, OBJECT_END, STRING_END};

type Result<T, E = de::value::Error> = std::result::Result<T, E>;

pub struct TokenSerializer<W> {
    writer: W,
    expected: Expected,
    current_key: Option<String>,
    depth: usize,
}

impl<W> TokenSerializer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            expected: Expected::Value,
            current_key: None,
            depth: 0,
        }
    }

    pub fn emit_object_start(&mut self) -> Result<()> {
        self.assert(Expected::Value)?;

        self.depth += 1;
        self.write_current_key(ValueType::Object)?;

        self.expected = Expected::KeyOrEndObject;

        Ok(())
    }

    pub fn emit_key(&mut self, key: String) -> Result<()> {
        self.assert(Expected::KeyOrEndObject)?;

        self.current_key = Some(key);

        self.expected = Expected::Value;

        Ok(())
    }

    pub fn emit_object_end(&mut self) -> Result<()> {
        self.assert(Expected::KeyOrEndObject)?;

        self.write_end_object()?;

        self.depth -= 1;

        self.end_of_value();

        Ok(())
    }

    pub fn emit_string(&mut self, string: &str) -> Result<()> {
        self.assert(Expected::Value)?;

        self.write_current_key(ValueType::String)?;
        self.write_string(string)?;

        self.end_of_value();

        Ok(())
    }

    pub fn emit_int(&mut self, value: u32) -> Result<()> {
        self.assert(Expected::Value)?;

        self.write_current_key(ValueType::Int)?;
        self.write_int(value)?;

        self.end_of_value();

        Ok(())
    }

    fn end_of_value(&mut self) {
        if self.depth == 0 {
            self.expected = Expected::End;
        } else {
            self.expected = Expected::KeyOrEndObject;
        }
    }

    fn assert(&self, got: Expected) -> Result<()> {
        if self.expected != got {
            return Err(ser::Error::custom(format!(
                "Expected {:?}, but got {:?}",
                self.expected, got
            )));
        }

        Ok(())
    }

    fn write_current_key(&mut self, value_type: ValueType) -> Result<()> {
        let key = match self.current_key.take() {
            Some(key) => key,
            None => return Ok(()),
        };

        self.write_value_type(value_type)?;
        self.write_string(&key)?;

        Ok(())
    }

    fn write_value_type(&mut self, value_type: ValueType) -> Result<()> {
        self.writer.write_all(&[value_type as u8]).map_err(ser::Error::custom)?;

        Ok(())
    }

    fn write_string(&mut self, value: &str) -> Result<()> {
        if value.as_bytes().contains(&STRING_END) {
            return Err(ser::Error::custom("string must not contain NULL"));
        }

        self.writer
            .write_all(value.as_bytes())
            .map_err(ser::Error::custom)?;
        self.writer
            .write_all(&[STRING_END])
            .map_err(ser::Error::custom)?;

        Ok(())
    }

    fn write_int(&mut self, value: u32) -> Result<()> {
        self.writer
            .write_all(&value.to_le_bytes())
            .map_err(ser::Error::custom)?;

        Ok(())
    }

    fn write_end_object(&mut self) -> Result<()> {
        self.writer
            .write_all(&[OBJECT_END])
            .map_err(ser::Error::custom)?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Expected {
    End,
    KeyOrEndObject,
    Value,
}
