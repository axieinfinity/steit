use std::io;

use iowrap::Eof;

use crate::{varint::Varint, wire_type::WireType};

pub trait Deserialize: Default + WireType {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

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
