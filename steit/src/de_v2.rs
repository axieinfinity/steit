use std::io;

use super::{
    reader::Reader,
    wire_format::{HasWireType, WireTypeV2},
};

pub trait MergeV2: HasWireType {
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()>;
}

pub trait MergeNested: HasWireType {
    fn merge_nested_v2(
        &mut self,
        wire_type: WireTypeV2,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()>;
}

impl<T: MergeV2> MergeNested for T {
    #[inline]
    fn merge_nested_v2(
        &mut self,
        wire_type: WireTypeV2,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()> {
        if wire_type != Self::WIRE_TYPE {
            return reader.skip_field(wire_type);
        }

        match wire_type {
            WireTypeV2::Varint => self.merge_v2(reader),
            WireTypeV2::Sized => self.merge_v2(&mut reader.nested()?),
        }
    }
}

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
