use std::io;

/// Wire type occupies three bits.
pub const WIRE_TYPE_BITS: u32 = 3;

/// This mask can be applied to obtain wire type.
pub const WIRE_TYPE_MASK: u32 = (1u32 << WIRE_TYPE_BITS) - 1;

/// Maximum possible field number.
pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WireTypeV2 {
    Varint = 0,
    Sized = 2,
}

impl WireTypeV2 {
    pub fn from_value(value: u32) -> io::Result<Self> {
        match value {
            0 => Ok(WireTypeV2::Varint),
            2 => Ok(WireTypeV2::Sized),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("illegal wire type {}", value),
            )),
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub fn tag(self, field_number: u32) -> io::Result<u32> {
        tag(field_number, self)
    }
}

pub trait HasWireType {
    const WIRE_TYPE: WireTypeV2;

    #[inline]
    fn wire_type(&self) -> WireTypeV2 {
        Self::WIRE_TYPE
    }

    #[inline]
    fn tag(&self, field_number: u32) -> io::Result<u32> {
        tag(field_number, Self::WIRE_TYPE)
    }
}

pub fn validate_field_number(field_number: u32) -> io::Result<()> {
    if field_number > FIELD_NUMBER_MAX {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "field number must not be greater than 2^29 - 1, got {}",
                field_number,
            ),
        ));
    }

    Ok(())
}

pub fn parse_tag(value: u32) -> io::Result<(u32, WireTypeV2)> {
    let wire_type = WireTypeV2::from_value(value & WIRE_TYPE_MASK)?;
    let field_number = value >> WIRE_TYPE_BITS;
    validate_field_number(field_number)?;
    Ok((field_number, wire_type))
}

pub fn tag(field_number: u32, wire_type: WireTypeV2) -> io::Result<u32> {
    validate_field_number(field_number)?;
    Ok(field_number << WIRE_TYPE_BITS | wire_type.value() as u32)
}
