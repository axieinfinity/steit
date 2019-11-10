use super::varint::Varint;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_SIZED: u8 = 2;

pub trait WireType {
    const WIRE_TYPE: u8;

    #[inline]
    fn wire_type(&self) -> u8 {
        Self::WIRE_TYPE
    }
}

impl<T: Varint> WireType for T {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
}

#[inline]
pub fn key(tag: u16, wire_type: u8) -> u32 {
    (tag as u32) << 3 | wire_type as u32
}

#[inline]
pub fn split_key(key: u32) -> (u16, u8) {
    let tag = (key >> 3) as u16;
    let wire_type = (key & 7) as u8;
    (tag, wire_type)
}
