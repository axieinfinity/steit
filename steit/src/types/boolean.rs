use std::io::{self, Read};

use crate::{
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Eof, Merge, Serialize,
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
        writer.write(&[*self as u8])?;
        Ok(())
    }
}

impl Merge for bool {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        let mut buf = [0];
        reader.read_exact(&mut buf)?;
        *self = buf[0] != 0;
        Ok(())
    }
}

impl Varint for bool {}
