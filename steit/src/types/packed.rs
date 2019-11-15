use std::io;

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Eof, Merge, Serialize,
};

use super::varint::Varint;

impl<T: Varint> WireType for Vec<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Varint> Serialize for Vec<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for item in self {
            size += item.compute_size_nested(None);
        }

        size
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            item.serialize_nested_with_cached_size(None, writer)?;
        }

        Ok(())
    }
}

impl<T: Varint> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = T::deserialize_nested(reader)?;
            self.push(item);
        }

        Ok(())
    }
}
