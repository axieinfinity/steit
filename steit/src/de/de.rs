use std::io;

use crate::wire_fmt::{HasWireType, WireType};

use super::reader::Reader;

pub trait Deserialize: Default + HasWireType {
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()>;

    fn deserialize(reader: &mut Reader<impl io::Read>) -> io::Result<Self> {
        let mut value = Self::default();
        value.merge(reader)?;
        Ok(value)
    }

    fn merge_nested(
        &mut self,
        wire_type: WireType,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()> {
        if wire_type != Self::WIRE_TYPE {
            return reader.skip_field(wire_type);
        }

        match wire_type {
            WireType::Varint => self.merge(reader),
            WireType::Sized => self.merge(&mut reader.nested()?),
        }
    }

    fn deserialize_nested(
        wire_type: WireType,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<Self> {
        let mut value = Self::default();
        value.merge_nested(wire_type, reader)?;
        Ok(value)
    }
}
