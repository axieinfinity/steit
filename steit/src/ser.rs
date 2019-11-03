use std::io;

use crate::varint::Varint;
use std::io::{Error, Write};

pub trait Serialize {
    fn wire_type(&self) -> u8 {
        2
    }

    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: Varint> Serialize for T {
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
