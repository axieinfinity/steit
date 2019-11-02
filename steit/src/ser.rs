use std::io;

use super::varint::Varint;

pub trait Serialize {
    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: Varint> Serialize for T {
    fn size(&self) -> u32 {
        Varint::size(self) as u32
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        Varint::serialize(self, writer)
    }
}
