use std::io;

use crate::wire_format::{HasWireType, WireTypeV2};

use super::reader::Reader;

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
