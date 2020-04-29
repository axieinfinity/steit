use std::{
    fmt,
    io::{self, Read},
};

use crate::{
    de_v2::{DeserializeV2, Reader},
    gen::{FieldTypeV2, IsFieldTypeV2},
    rt::SizeCache,
    ser_v2::SerializeV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

#[derive(PartialEq, Eq, Default, Hash)]
pub struct BytesV2(Vec<u8>);

impl BytesV2 {
    #[inline]
    pub fn from_raw(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn from_value(value: &impl SerializeV2) -> Self {
        let mut bytes = Vec::new();
        value.serialize_v2(&mut bytes).unwrap();
        Self(bytes)
    }

    #[inline]
    pub fn into_raw(self) -> Vec<u8> {
        self.0
    }
}

impl fmt::Debug for BytesV2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl HasWireType for BytesV2 {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl SerializeV2 for BytesV2 {
    #[inline]
    fn compute_size_v2(&self) -> u32 {
        self.0.len() as u32
    }

    #[inline]
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        None
    }
}

impl DeserializeV2 for BytesV2 {
    #[inline]
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.0)?;
        Ok(())
    }
}

impl IsFieldTypeV2 for BytesV2 {
    const FIELD_TYPE: &'static FieldTypeV2 = &FieldTypeV2::MetaRef("Bytes");
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util_v2::{assert_merge, assert_serialize, assert_serialize_nested, assert_size},
    };

    use super::BytesV2;

    test_case!(size_01: assert_size; BytesV2::from_value(&None::<u8>) => 0);
    test_case!(size_02: assert_size; BytesV2::from_value(&Some(1337)) => 2);
    test_case!(size_03: assert_size; BytesV2::from_value(&0) => 1);
    // test_case!(size_04: assert_size; BytesV2::from_value(&Foo::new()) => 0);
    // test_case!(size_05: assert_size; BytesV2::from_value(&Foo::with(-1, -1)) => 4);

    test_case!(serialize_01: assert_serialize; BytesV2::from_value(&None::<u8>) => &[]);
    test_case!(serialize_02: assert_serialize; BytesV2::from_value(&Some(1337)) => &[242, 20]);
    test_case!(serialize_03: assert_serialize; BytesV2::from_value(&0) => &[0]);
    // test_case!(serialize_04: assert_serialize; BytesV2::from_value(&Foo::new()) => &[]);
    // test_case!(serialize_05: assert_serialize; BytesV2::from_value(&Foo::with(-1, -1)) => &[0, 1, 8, 1]);

    test_case!(serialize_nested_01: assert_serialize_nested; BytesV2::from_value(&None::<u8>), None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; BytesV2::from_value(&Some(1)), None => &[1, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; BytesV2::from_value(&1), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_04: assert_serialize_nested; BytesV2::from_value(&None::<u8>), Some(10) => &[]);
    test_case!(serialize_nested_05: assert_serialize_nested; BytesV2::from_value(&Some(1)), Some(10) => &[82, 1, 2]);
    // test_case!(serialize_nested_06: assert_serialize_nested; BytesV2::from_value(&Some(Foo::new())), Some(10) => &[82, 1, 0]);
    // test_case!(serialize_nested_07: assert_serialize_nested; BytesV2::from_value(&Some(Foo::with(-1, -2))), Some(10) => &[82, 5, 4, 0, 1, 8, 3]);

    test_case!(merge_01: assert_merge; BytesV2::from_value(&None::<u8>), &[242, 20] => BytesV2::from_value(&Some(1337)));
    // test_case!(merge_02: assert_merge; BytesV2::from_value(&Foo::with(-1, -1)), &[8, 3] => BytesV2::with_raw(vec![0, 1, 8, 1, 8, 3]));
}
