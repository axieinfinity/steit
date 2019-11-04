use crate::varint::Varint;

pub trait WireType {
    const WIRE_TYPE: u8 = 2;
}

impl<T: Varint> WireType for T {
    const WIRE_TYPE: u8 = 0;
}

#[inline]
pub fn wire_type<T: WireType>(_: &T) -> u8 {
    T::WIRE_TYPE
}
