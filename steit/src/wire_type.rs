use crate::varint::Varint;

pub trait WireType {
    const WIRE_TYPE: u8;
}

impl<T: Varint> WireType for T {
    const WIRE_TYPE: u8 = 0;
}

pub fn wire_type<T: WireType>(_: &T) -> u8 {
    T::WIRE_TYPE
}
