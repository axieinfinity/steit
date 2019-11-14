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
    fn size(&self) -> u32 {
        let mut size = 0;

        for item in self {
            size += item.size_nested(None);
        }

        size
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            item.serialize_nested(None, writer)?;
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
