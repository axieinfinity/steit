use std::io;

use crate::{
    ser_v2::SerializePrimitive,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl HasWireType for &str {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl SerializePrimitive for &str {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.len() as u32
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(self.as_bytes())
    }
}
