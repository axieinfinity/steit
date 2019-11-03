use std::io;

use crate::varint::Varint;

pub trait Serialize {
    #[inline]
    fn wire_type(&self) -> u8 {
        2
    }

    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: Varint> Serialize for T {
    #[inline]
    fn wire_type(&self) -> u8 {
        0
    }

    fn size(&self) -> u32 {
        Varint::size(self) as u32
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        Varint::serialize(self, writer)
    }
}

impl<T: Varint> Serialize for Vec<T> {
    fn size(&self) -> u32 {
        self.iter()
            .fold(0, |size, item| size + Varint::size(item) as u32)
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            Varint::serialize(item, writer)?;
        }

        Ok(())
    }
}
