mod de;
mod ser;

pub use de::from_reader;
use enum_primitive_derive::Primitive;

const STRING_END: u8 = 0x00;
const OBJECT_END: u8 = 0x08;

#[derive(Primitive)]
enum ValueType {
    Object = 0x00,
    String = 0x01,
    Int = 0x02,
}
