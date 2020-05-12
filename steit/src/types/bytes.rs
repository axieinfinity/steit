use std::{
    fmt,
    io::{self, Read},
};

use crate::{
    de::{Deserialize, Reader},
    meta::{HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::SizeCache,
    ser::Serialize,
    wire_fmt::{HasWireType, WireType},
};

#[derive(PartialEq, Eq, Default, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    #[inline]
    pub fn from_raw(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn from_value(value: &impl Serialize) -> Self {
        let mut bytes = Vec::new();
        value.serialize(&mut bytes).unwrap();
        Self(bytes)
    }

    #[inline]
    pub fn into_raw(self) -> Vec<u8> {
        self.0
    }
}

impl fmt::Debug for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl HasWireType for Bytes {
    const WIRE_TYPE: WireType = WireType::Sized;
}

impl Serialize for Bytes {
    #[inline]
    fn compute_size(&self) -> u32 {
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

impl Deserialize for Bytes {
    #[inline]
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.0)?;
        Ok(())
    }
}

impl HasMeta for Bytes {
    const NAME: &'static NameMeta = &NameMeta::new("Bytes");
    const TYPE: &'static TypeMeta = &TypeMeta::Ref(Self::NAME, &[]);
    const LINK: &'static MetaLink = &MetaLink {
        r#type: Self::TYPE,
        msg: None,
        links: || &[],
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size, Foo},
    };

    use super::Bytes;

    test_case!(size_01: assert_size; Bytes::from_value(&None::<u8>) => 0);
    test_case!(size_02: assert_size; Bytes::from_value(&Some(1337)) => 2);
    test_case!(size_03: assert_size; Bytes::from_value(&0) => 1);
    test_case!(size_04: assert_size; Bytes::from_value(&Foo::empty()) => 0);
    test_case!(size_05: assert_size; Bytes::from_value(&Foo::new(-1, -1)) => 4);

    test_case!(serialize_01: assert_serialize; Bytes::from_value(&None::<u8>) => &[]);
    test_case!(serialize_02: assert_serialize; Bytes::from_value(&Some(1337)) => &[242, 20]);
    test_case!(serialize_03: assert_serialize; Bytes::from_value(&0) => &[0]);
    test_case!(serialize_04: assert_serialize; Bytes::from_value(&Foo::empty()) => &[]);
    test_case!(serialize_05: assert_serialize; Bytes::from_value(&Foo::new(-1, -1)) => &[0, 1, 8, 1]);

    test_case!(serialize_nested_01: assert_serialize_nested; Bytes::from_value(&None::<u8>), None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; Bytes::from_value(&Some(1)), None => &[1, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; Bytes::from_value(&1), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_04: assert_serialize_nested; Bytes::from_value(&None::<u8>), Some(10) => &[]);
    test_case!(serialize_nested_05: assert_serialize_nested; Bytes::from_value(&Some(1)), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_06: assert_serialize_nested; Bytes::from_value(&Some(Foo::empty())), Some(10) => &[82, 1, 0]);
    test_case!(serialize_nested_07: assert_serialize_nested; Bytes::from_value(&Some(Foo::new(-1, -2))), Some(10) => &[82, 5, 4, 0, 1, 8, 3]);

    test_case!(merge_01: assert_merge; Bytes::from_value(&None::<u8>), &[242, 20] => Bytes::from_value(&Some(1337)));
    test_case!(merge_02: assert_merge; Bytes::from_value(&Foo::new(-1, -1)), &[8, 3] => Bytes::from_raw(vec![0, 1, 8, 1, 8, 3]));
}
