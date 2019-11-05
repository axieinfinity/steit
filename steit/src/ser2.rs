use std::io;

use crate::{varint::Varint, wire_type::WireType};

pub trait Serialize: WireType {
    #[inline]
    fn non_empty(&self) -> bool {
        self.size() > 0
    }

    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: Default + Eq + Varint + WireType> Serialize for T {
    #[inline]
    fn non_empty(&self) -> bool {
        *self != Self::default()
    }

    #[inline]
    fn size(&self) -> u32 {
        Varint::size(self) as u32
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        Varint::serialize(self, writer)
    }
}
