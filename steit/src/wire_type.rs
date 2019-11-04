use crate::varint::Varint;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_SIZED: u8 = 2;

pub trait WireType {
    const WIRE_TYPE: u8;
}

impl<T: Varint> WireType for T {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
}

#[inline]
pub fn wire_type<T: WireType>(_: &T) -> u8 {
    T::WIRE_TYPE
}
