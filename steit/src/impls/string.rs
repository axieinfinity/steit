use std::io::{self, Read};

use crate::{
    de::{Deserialize, Reader},
    impl_meta_primitive, impl_serialize_primitive,
    wire_fmt::{HasWireType, WireType},
};

impl HasWireType for String {
    const WIRE_TYPE: WireType = WireType::Sized;
}

fn compute_size(value: &str) -> u32 {
    value.len() as u32
}

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

fn from_utf8(bytes: Vec<u8>) -> io::Result<String> {
    String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size},
    };

    test_case!(size_01: assert_size; "234" => 3);
    test_case!(size_02: assert_size; "π" => 2);
    test_case!(size_03: assert_size; "π is roughly 3.14" => 18);

    test_case!(serialize_01: assert_serialize; "234" => &[50, 51, 52]);
    test_case!(serialize_02: assert_serialize; "π" => &[207, 128]);

    test_case!(serialize_nested_01: assert_serialize_nested; "33", None => &[2, 51, 51]);
    test_case!(serialize_nested_02: assert_serialize_nested; "π", None => &[2, 207, 128]);
    test_case!(serialize_nested_03: assert_serialize_nested; "33", Some(10) => &[82, 2, 51, 51]);
    test_case!(serialize_nested_04: assert_serialize_nested; "π", Some(10) => &[82, 2, 207, 128]);

    test_case!(merge_01: assert_merge; "foo".to_string(), &[51, 51] => "33".to_string());
    test_case!(merge_02: assert_merge; "bar".to_string(), &[207, 128] => "π".to_string());
}
