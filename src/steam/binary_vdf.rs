use enum_primitive_derive::Primitive;

pub use self::{
    de::{from_bytes, from_reader},
    ser::{to_bytes, to_writer},
    token_deserializer::TokenDeserializer,
    token_serializer::TokenSerializer,
};

mod de;
mod ser;
pub mod token_deserializer;
mod token_serializer;

const STRING_END: u8 = 0x00;
const OBJECT_END: u8 = 0x08;

#[derive(Primitive, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ValueType {
    Object = 0x00,
    String = 0x01,
    Int = 0x02,
}
