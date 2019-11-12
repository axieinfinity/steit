use std::io::{self, Read};

use super::{
    types::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
    Eof,
};

pub trait Merge: WireType {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

    #[inline]
    fn merge_nested(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            let size = u64::deserialize(reader)?;
            let reader = &mut Eof::new(reader.by_ref().take(size));
            self.merge(reader)
        } else {
            self.merge(reader)
        }
    }
}

impl<T: Varint> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = T::deserialize_nested(reader)?;
            self.push(item);
        }

        Ok(())
    }
}

pub trait Deserialize: Default + Merge {
    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        // We use `Self::` since surprisingly `Default::` leaves us with an unknown type.
        let mut value = Self::default();
        value.merge(reader)?;
        Ok(value)
    }

    #[inline]
    fn deserialize_nested(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        let mut value = Self::default();
        value.merge_nested(reader)?;
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
