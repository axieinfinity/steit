use std::io::{self, Read};

use iowrap::Eof;

use crate::{
    varint::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
};

pub trait Deserialize: Default + WireType {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

    #[inline]
    fn merge_nested(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            // TODO: Remove `as Deserialize` after refactoring `Varint`
            let size = <u64 as Deserialize>::deserialize(reader)?;
            let reader = &mut Eof::new(reader.by_ref().take(size));
            self.merge(reader)
        } else {
            self.merge(reader)
        }
    }

    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        // We use `Self::` since surprisingly `Default::` leaves us with an unknown type.
        let mut value = Self::default();
        value.merge(reader)?;
        Ok(value)
    }
}

impl<T: Default + Varint + WireType> Deserialize for T {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }

    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        Varint::deserialize(reader)
    }
}
