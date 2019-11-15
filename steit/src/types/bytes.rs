use std::{
    fmt,
    io::{self, Read},
};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Eof, Merge, Serialize,
};

#[derive(Default)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    #[inline]
    pub fn new(value: &impl Serialize) -> Self {
        let mut bytes = Vec::new();
        value.serialize(&mut bytes).unwrap();
        Self { bytes }
    }

    #[inline]
    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl fmt::Debug for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bytes {:?}", self.bytes)
    }
}

impl WireType for Bytes {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Serialize for Bytes {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.bytes.compute_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.bytes.serialize_with_cached_size(writer)
    }
}

impl Merge for Bytes {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.bytes)?;
        Ok(())
    }
}
