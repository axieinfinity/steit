use std::io;

use crate::{
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Deserialize, Eof, Merge, Serialize,
};

use super::varint::Varint;

impl WireType for bool {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
}

impl Serialize for bool {
    #[inline]
    fn size(&self) -> u32 {
        1
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl Merge for bool {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = u64::deserialize(reader)? != 0;
        Ok(())
    }
}

impl Varint for bool {}
