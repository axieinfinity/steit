use std::io;

use crate::wire_format::WireTypeV2;

use super::{
    merge::{MergeNested, MergeV2},
    reader::Reader,
};

pub trait DeserializeV2: Default + MergeV2 {
    #[inline]
    fn deserialize_v2(reader: &mut Reader<impl io::Read>) -> io::Result<Self> {
        let mut value = Self::default();
        value.merge_v2(reader)?;
        Ok(value)
    }
}

impl<T: Default + MergeV2> DeserializeV2 for T {}

pub trait DeserializeNested: Default + MergeNested {
    #[inline]
    fn deserialize_nested_v2(
        wire_type: WireTypeV2,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<Self> {
        let mut value = Self::default();
        value.merge_nested_v2(wire_type, reader)?;
        Ok(value)
    }
}

impl<T: Default + MergeNested> DeserializeNested for T {}
