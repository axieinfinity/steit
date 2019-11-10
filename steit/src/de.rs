use std::io::{self, Read};

use iowrap::Eof;

use super::{
    merge::Merge,
    wire_type::{WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
};

pub trait Deserialize: Default + Merge {
    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        // We use `Self::` since surprisingly `Default::` leaves us with an unknown type.
        let mut value = Self::default();
        value.merge(reader)?;
        Ok(value)
    }
}

impl<T: Default + Merge> Deserialize for T {}

#[inline]
pub fn exhaust_nested(tag: u16, wire_type: u8, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
    match wire_type {
        WIRE_TYPE_VARINT => {
            u8::deserialize(reader)?;
        }

        WIRE_TYPE_SIZED => {
            let size = u64::deserialize(reader)?;
            let mut buf = Vec::new();
            reader.by_ref().take(size).read_to_end(&mut buf)?;
        }

        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected tag {} or wire type {}", tag, wire_type),
            ))
        }
    }

    Ok(())
}
