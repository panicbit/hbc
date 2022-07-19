use std::io::BufRead;

use anyhow::{Context, Result};
use num_traits::FromPrimitive;
use serde::de::{self, DeserializeOwned, MapAccess};
use serde::forward_to_deserialize_any;

use super::{ValueType, OBJECT_END, STRING_END};

pub fn from_reader<D: DeserializeOwned, R: BufRead>(reader: R) -> Result<D, de::value::Error> {
    let mut de = Deserializer {
        reader,
        value_type: ValueType::Object,
    };

    D::deserialize(&mut de)
}

struct Deserializer<R> {
    reader: R,
    value_type: ValueType,
}

impl<R> Deserializer<R>
where
    R: BufRead,
{
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

impl<'de, R> serde::Deserializer<'de> for &mut Deserializer<R>
where
    R: BufRead,
{
    type Error = de::value::Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value_type {
            ValueType::Object => visitor.visit_map(&mut self),
            ValueType::String => {
                let string = self.read_string().map_err(de::Error::custom)?;

                visitor.visit_string(string)
            }
            ValueType::Int => {
                let int = self.read_int().map_err(de::Error::custom)?;

                visitor.visit_u32(int)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = self.read_int().map_err(de::Error::custom)?;
        let value = match value {
            0 => false,
            1 => true,
            _ => return Err(de::Error::custom(format!("invalid bool value: {}", value))),
        };

        visitor.visit_bool(value)
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, R> MapAccess<'de> for Deserializer<R>
where
    R: BufRead,
{
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let value_type = match self.read_value_type().map_err(de::Error::custom)? {
            Some(value_type) => value_type,
            None => return Ok(None),
        };

        self.value_type = ValueType::String;
        let key = seed.deserialize(&mut *self)?;
        self.value_type = value_type;

        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}
