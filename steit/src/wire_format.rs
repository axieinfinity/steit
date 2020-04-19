#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WireTypeV2 {
    Varint = 0,
    Sized = 2,
}

impl WireTypeV2 {
    pub fn from_value(value: u32) -> Result<Self, String> {
        match value {
            0 => Ok(WireTypeV2::Varint),
            2 => Ok(WireTypeV2::Sized),
            _ => Err(format!("illegal wire type {}", value)),
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

pub trait HasWireType {
    const WIRE_TYPE: WireTypeV2;

    #[inline]
    fn wire_type(&self) -> WireTypeV2 {
        Self::WIRE_TYPE
    }
}
