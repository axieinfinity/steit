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
                io::ErrorKind::InvalidInput,
                format!("illegal wire type {}", value),
            )),
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub fn tag(self, field_number: u32) -> Tag {
        Tag::new(field_number, self)
    }
}

pub trait HasWireType {
    const WIRE_TYPE: WireTypeV2;

    #[inline]
    fn wire_type(&self) -> WireTypeV2 {
        Self::WIRE_TYPE
    }

    #[inline]
    fn tag(&self, field_number: u32) -> Tag {
        self.wire_type().tag(field_number)
    }
}

pub struct Tag {
    field_number: u32,
    wire_type: WireTypeV2,
}

impl Tag {
    #[inline]
    pub fn new(field_number: u32, wire_type: WireTypeV2) -> Tag {
        assert!(
            field_number >= 1 && field_number <= FIELD_NUMBER_MAX,
            "field number must be from 1 to 2^29 - 1, inclusive",
        );

        Self {
            field_number,
            wire_type,
        }
    }

    pub fn from_value(value: u32) -> io::Result<Self> {
        let wire_type = WireTypeV2::from_value(value & WIRE_TYPE_MASK)?;
        let field_number = value >> WIRE_TYPE_BITS;

        if field_number == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "field number must not be 0",
            ));
        }

        if field_number > FIELD_NUMBER_MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "field number must not be greater than 2^29 - 1, got {}",
                    field_number,
                ),
            ));
        }

        Ok(Self {
            field_number,
            wire_type,
        })
    }

    #[inline]
    pub fn value(&self) -> u32 {
        self.field_number << WIRE_TYPE_BITS | self.wire_type.value() as u32
    }

    #[inline]
    pub fn unpack(self) -> (u32, WireTypeV2) {
        (self.field_number, self.wire_type)
    }
}
