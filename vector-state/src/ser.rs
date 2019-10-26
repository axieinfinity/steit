use std::io;

use crate::varint::Varint;

pub trait Serialize {
    fn size(&self) -> u32;
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl<I> Serialize for I
where
    I: Varint,
{
    fn size(&self) -> u32 {
        Varint::size(self) as u32
    }

    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        Varint::serialize(self, writer)
    }
}
