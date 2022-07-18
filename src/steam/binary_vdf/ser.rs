use std::io::Write;

use serde::de;
use serde::ser::{self, Impossible};

#[derive(PartialEq, Debug)]
enum Position {
    Object,
    Key,
    Value,
}

struct Serializer<W> {
    writer: W,
    at_root: bool,
    position: Position,
    current_key: Option<String>,
}

impl<W> Serializer<W>
where
    W: Write,
{
    pub fn allow(&self, positions: &[Position]) -> Result<(), de::value::Error> {
        if positions.is_empty() {
            return Err(ser::Error::custom("cannot serialize type"));
        }

        if positions.contains(&self.position) {
            return Err(ser::Error::custom(format!(
                "cannot serialize type in {:?} position",
                self.position
            )));
        }

        Ok(())
    }
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = de::value::Error;

    type SerializeSeq = Impossible<(), de::value::Error>;
    type SerializeTuple = Impossible<(), de::value::Error>;
    type SerializeTupleStruct = Impossible<(), de::value::Error>;
    type SerializeTupleVariant = Impossible<(), de::value::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Impossible<(), de::value::Error>;
    type SerializeStructVariant = Impossible<(), de::value::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.allow(&[Position::Value])?;

        self.writer.write_all(&value.to_be_bytes()).map_err(de::Error::custom)?;

        self.position = Position::Key;

        Ok(())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.allow(&[Position::Key, Position::Value])?;

        if self.position == Position::Key {
            self.current_key = Some(value.to_owned());

            return Ok(())
        }

        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.allow(&[])
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.allow(&[])
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.allow(&[])
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.allow(&[])
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!();
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.allow(&[])
            .map(|_| -> Impossible<_, _> { unreachable!() })
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
        self.allow(&[Position::Key])?;
        key.serialize(&mut **self)?;
        self.position = Position::Object;

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.allow(&[Position::Value])?;
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[super::OBJECT_END]).map_err(de::Error::custom)?;

        Ok(())
    }
}
