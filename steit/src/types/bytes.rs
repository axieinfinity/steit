use std::io::{self, Read};

use crate::{
    impl_state_for_plain,
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Eof, Merge, Serialize, State,
};

#[derive(Default, Debug)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    #[inline]
    pub fn with_value(value: &impl Serialize) -> Self {
        let mut bytes = Vec::new();
        value.serialize(&mut bytes).unwrap();
        Self { bytes }
    }

    #[inline]
    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl WireType for Bytes {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Serialize for Bytes {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.bytes.len() as u32
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }
}

impl Merge for Bytes {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.bytes)?;
        Ok(())
    }
}

impl State for Bytes {
    impl_state_for_plain!("bytes");
}
