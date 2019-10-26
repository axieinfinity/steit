use std::io;

use varint::Varint;
use std::io::Error;

pub trait Deserialize: Sized {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl<I> Deserialize for I
    where
        I: Varint,
{
    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        Varint::deserialize(reader)
    }
}
