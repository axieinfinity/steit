use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize<R: io::Read>(&mut self, reader: &mut R) -> io::Result<()>;
}

impl<T: Varint> Deserialize for T {
    fn deserialize<R: io::Read>(&mut self, reader: &mut R) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }
}
