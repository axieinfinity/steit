use std::io;

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Deserialize, Eof, Merge, Serialize,
};

impl<T> WireType for Option<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Serialize> Serialize for Option<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        match self {
            Some(value) => value.compute_size_nested(None),
            None => 0,
        }
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Some(value) => value.serialize_nested_with_cached_size(None, writer),
            None => Ok(()),
        }
    }
}

impl<T: Deserialize> Merge for Option<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let value = T::deserialize_nested(reader)?;
            *self = Some(value);
        }

        Ok(())
    }
}
