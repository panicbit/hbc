use std::io::BufRead;

use anyhow::{Context, Result};
use serde::de::{self, DeserializeOwned, MapAccess, SeqAccess, IntoDeserializer};
use serde::{forward_to_deserialize_any};

use super::token_deserializer::Token;
use super::TokenDeserializer;

pub fn from_reader<D: DeserializeOwned, R: BufRead>(reader: R) -> Result<D, de::value::Error> {
    let mut de = Deserializer {
        token_deserializer: TokenDeserializer::new(reader),
    };

    D::deserialize(&mut de)
}

pub fn from_bytes<D: DeserializeOwned>(bytes: &[u8]) -> Result<D, de::value::Error> {
    from_reader(bytes)
}

struct Deserializer<R> {
    token_deserializer: TokenDeserializer<R>,
}

impl<R> Deserializer<R>
where
    R: BufRead,
{
    fn expect_token(&mut self) -> Result<Token, de::value::Error> {
        let token = self.token_deserializer.next().context("no more data to deserialize")
            .map_err(de::Error::custom)?
            .map_err(de::Error::custom)?;

        Ok(token)
    }

    fn expect_int(&mut self) -> Result<u32, de::value::Error> {
        match self.expect_token()? {
            Token::Int(value) => Ok(value),
            token => unexpected_token(&token),
        }
    }

    fn expect_object_start(&mut self) -> Result<(), de::value::Error> {
        match self.expect_token()? {
            Token::ObjectStart => Ok(()),
            token => unexpected_token(&token),
        }
    }

    fn expect_key_or_object_end(&mut self) -> Result<Option<String>, de::value::Error> {
        match self.expect_token()? {
            Token::Key(key) => Ok(Some(key)),
            Token::ObjectEnd => Ok(None),
            token => unexpected_token(&token),
        }
    }
}

fn unexpected_token<T>(token: &Token) -> Result<T, de::value::Error> {
    Err(de::Error::custom(format!("unexpected token: {:?}", token)))
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
        let value = match self.expect_token()? {
            Token::ObjectStart => visitor.visit_map(self)?,
            Token::Int(value) => visitor.visit_u32(value)?,
            Token::String(value) => visitor.visit_string(value)?,
            token => return unexpected_token(&token),
        };

        Ok(value)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.expect_int()? {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            value => return Err(de::Error::custom(format!("invalid bool value: {}", value))),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.expect_object_start()?;
        visitor.visit_seq(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
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
        let key = match self.expect_key_or_object_end()? {
            Some(key) => key,
            None => return Ok(None),
        };

        let key = seed.deserialize(key.into_deserializer())?;

        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

impl<'de, R> SeqAccess<'de> for Deserializer<R>
where
    R: BufRead,
{
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.expect_key_or_object_end()?.is_none() {
            return Ok(None);
        }

        seed.deserialize(self).map(Some)
    }
}
