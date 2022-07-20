use std::io::Write;

use serde::ser::{self, Impossible};
use serde::{de, Serialize};

use super::{ValueType, STRING_END};

pub fn to_writer<S: Serialize, W: Write>(writer: W, value: &S) -> Result<(), de::value::Error> {
    let mut ser = Serializer {
        writer,
        position: Position::Other,
        current_key: None,
    };

    value.serialize(&mut ser)
}

pub fn to_bytes<S: Serialize>(value: &S) -> Result<Vec<u8>, de::value::Error> {
    let mut bytes = Vec::new();

    to_writer(&mut bytes, value)?;

    Ok(bytes)
}

#[derive(PartialEq, Debug)]
enum Position {
    Key,
    Value,
    Other,
}

struct Serializer<W> {
    writer: W,
    position: Position,
    current_key: Option<String>,
}

impl<W> Serializer<W>
where
    W: Write,
{
    pub fn require_not_in_key(&self) -> Result<(), de::value::Error> {
        if self.position == Position::Key {
            return Err(de::Error::custom("keys only support string types"));
        }

        Ok(())
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> Result<(), de::value::Error> {
        eprintln!("trying to write_bytes: {:?}", value);
        validate_bytes(value)?;
        self.write_bytes_unvalidated(value)?;

        Ok(())
    }

    pub fn write_bytes_unvalidated(&mut self, value: &[u8]) -> Result<(), de::value::Error> {
        self.writer.write_all(value).map_err(de::Error::custom)?;
        self.writer
            .write_all(&[STRING_END])
            .map_err(de::Error::custom)?;

        Ok(())
    }

    pub fn flush_key(&mut self, value_type: ValueType) -> Result<(), de::value::Error> {
        if self.position != Position::Value {
            return Ok(());
        }

        let key = match self.current_key.take() {
            Some(key) => key,
            None => return Err(ser::Error::custom("invalid state: missing key")),
        };

        self.writer
            .write_all(&[value_type as u8])
            .map_err(ser::Error::custom)?;
        self.write_bytes_unvalidated(key.as_bytes())?;

        Ok(())
    }
}

pub fn validate_bytes(value: &[u8]) -> Result<(), de::value::Error> {
    if value.contains(&STRING_END) {
        return Err(de::Error::custom(format!(
            "bytes and strings must not contain NULL: {:?}",
            value
        )));
    }

    Ok(())
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = de::value::Error;

    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeTuple = Impossible<(), de::value::Error>;
    type SerializeTupleStruct = Impossible<(), de::value::Error>;
    type SerializeTupleVariant = Impossible<(), de::value::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), de::value::Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "binary vdf does not support 64 bit integers",
        ))
    }

    fn serialize_i128(self, _: i128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "binary vdf does not support 128 bit integers",
        ))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.require_not_in_key()?;
        self.flush_key(ValueType::Int)?;
        self.writer
            .write_all(&value.to_le_bytes())
            .map_err(de::Error::custom)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "binary vdf does not support 64 bit integers",
        ))
    }

    fn serialize_u128(self, _: u128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "binary vdf does not support 128 bit integers",
        ))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value.to_bits())
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "binary vdf does not support 64 bit floats",
        ))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value.into())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        validate_bytes(value.as_bytes())?;

        if self.position == Position::Key {
            self.current_key = Some(value.into());
            return Ok(());
        }

        self.flush_key(ValueType::String)?;
        self.write_bytes_unvalidated(value.as_bytes())?;

        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.flush_key(ValueType::String)?;
        self.write_bytes(value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(de::Error::custom("serializing Option is not supported"))
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(de::Error::custom("serializing Option is not supported"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(de::Error::custom("serializing unit is not supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(de::Error::custom(
            "serializing unit structs is not supported",
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(de::Error::custom(
            "serializing unit varints is not supported",
        ))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(de::Error::custom(
            "serializing newtype structs is not supported",
        ))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(de::Error::custom(
            "serializing newtype variants is not supported",
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.require_not_in_key()?;
        self.flush_key(ValueType::Object)?;

        Ok(SerializeSeq { i: 0, ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(de::Error::custom("serializing tuple is not supported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(de::Error::custom(
            "serializing tuple struct is not supported",
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(de::Error::custom(
            "serializing tuple variant is not supported",
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.require_not_in_key()?;
        self.flush_key(ValueType::Object)?;

        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.require_not_in_key()?;
        self.flush_key(ValueType::Object)?;

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(de::Error::custom(
            "serializing struct variant is not supported",
        ))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = de::value::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.position = Position::Key;
        key.serialize(&mut **self)?;

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.position = Position::Value;
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.position == Position::Key {
            return Err(de::Error::custom("incomplete map"));
        }

        self.position = Position::Other;

        self.writer
            .write_all(&[super::OBJECT_END])
            .map_err(de::Error::custom)?;

        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = de::value::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.position = Position::Key;
        key.serialize(&mut **self)?;

        self.position = Position::Value;
        value.serialize(&mut **self)?;

        self.position = Position::Other;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.position != Position::Other {
            return Err(de::Error::custom("incomplete struct"));
        }

        self.writer
            .write_all(&[super::OBJECT_END])
            .map_err(de::Error::custom)?;

        Ok(())
    }
}

struct SerializeSeq<'a, W> {
    ser: &'a mut Serializer<W>,
    i: usize,
}

impl<'a, W> ser::SerializeSeq for SerializeSeq<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = de::value::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser.current_key = Some(self.i.to_string());

        self.ser.position = Position::Value;
        value.serialize(&mut *self.ser)?;

        self.i += 1;

        self.ser.position = Position::Other;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser
            .writer
            .write_all(&[super::OBJECT_END])
            .map_err(de::Error::custom)?;

        Ok(())
    }
}
