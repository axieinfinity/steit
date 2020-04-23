use std::io::{self, Read};

use crate::{
    de_v2::{DeserializeV2, Reader},
    ser_v2::SerializePrimitive,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl HasWireType for String {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl SerializePrimitive for String {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.len() as u32
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl DeserializeV2 for String {
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        *self = from_utf8(bytes)?;
        Ok(())
    }
}

#[inline]
fn from_utf8(bytes: Vec<u8>) -> io::Result<String> {
    String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
