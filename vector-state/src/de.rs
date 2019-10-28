use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl<I: Varint> Deserialize for I {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Varint::deserialize(reader)
    }
}
