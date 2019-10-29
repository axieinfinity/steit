use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize<R: io::Read>(reader: &mut R, target: &mut Self) -> io::Result<()>;
}

impl<I: Varint> Deserialize for I {
    fn deserialize<R: io::Read>(reader: &mut R, target: &mut Self) -> io::Result<()> {
        *target = Varint::deserialize(reader)?;
        Ok(())
    }
}
