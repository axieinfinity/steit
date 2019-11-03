use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize(&mut self, reader: &mut impl io::Read) -> io::Result<()>;
}

impl<T: Varint> Deserialize for T {
    fn deserialize(&mut self, reader: &mut impl io::Read) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }
}
