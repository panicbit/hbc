use enum_primitive_derive::Primitive;

pub use self::de::{from_reader, from_bytes};
pub use self::ser::{to_writer, to_bytes};

mod de;
mod ser;

const STRING_END: u8 = 0x00;
const OBJECT_END: u8 = 0x08;

#[derive(Primitive)]
enum ValueType {
    Object = 0x00,
    String = 0x01,
    Int = 0x02,
}
