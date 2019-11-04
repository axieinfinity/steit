use std::io;

use crate::{varint::Varint, wire_type::WireType};

pub trait Serialize: WireType {
    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: Varint + WireType> Serialize for T {
    #[inline]
    fn size(&self) -> u32 {
        Varint::size(self) as u32
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        Varint::serialize(self, writer)
    }
}
