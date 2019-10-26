use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl<I> Deserialize for I
where
    I: Varint,
{
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Varint::deserialize(reader)
    }
}
