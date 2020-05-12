use std::io;

use crate::{
    impl_serialize_primitive,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl HasWireType for &str {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

#[inline]
fn compute_size(value: &str) -> u32 {
    value.len() as u32
}

#[inline]
fn serialize(value: &str, writer: &mut impl io::Write) -> io::Result<()> {
    writer.write_all(value.as_bytes())
}

impl_serialize_primitive!(&str, compute_size, serialize);
