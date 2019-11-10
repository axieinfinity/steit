use std::io::{self, Read};

use iowrap::Eof;

use crate::{
    varint::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
};

pub trait Merge: WireType {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

    #[inline]
    fn merge_nested(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            // TODO: Remove `as Varint` after refactoring `Varint`
            let size = <u64 as Varint>::deserialize(reader)?;
            let reader = &mut Eof::new(reader.by_ref().take(size));
            self.merge(reader)
        } else {
            self.merge(reader)
        }
    }
}

impl<T: Varint + WireType> Merge for T {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
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
}

impl<T: Default + Merge> Deserialize for T {}

pub fn exhaust_nested(tag: u16, wire_type: u8, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
    match wire_type {
        WIRE_TYPE_VARINT => {
            // TODO: Remove `as Varint` after refactoring `Varint`
            <u8 as Varint>::deserialize(reader)?;
        }

        WIRE_TYPE_SIZED => {
            // TODO: Remove `as Varint` after refactoring `Varint`
            let size = <u64 as Varint>::deserialize(reader)?;
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
