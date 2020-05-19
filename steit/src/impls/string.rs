use std::io::{self, Read};

use crate::{
    de::{Deserialize, Reader},
    impl_meta_primitive, impl_serialize_primitive,
    wire_fmt::{HasWireType, WireType},
};

impl HasWireType for String {
    const WIRE_TYPE: WireType = WireType::Sized;
}

#[inline]
fn compute_size(value: &str) -> u32 {
    value.len() as u32
}

#[inline]
fn serialize(value: &str, writer: &mut impl io::Write) -> io::Result<()> {
    writer.write_all(value.as_bytes())
}

impl_serialize_primitive!(String, compute_size, serialize);

impl Deserialize for String {
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        *self = from_utf8(bytes)?;
        Ok(())
    }
}

impl_meta_primitive!(String, "String");

#[inline]
fn from_utf8(bytes: Vec<u8>) -> io::Result<String> {
    String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
