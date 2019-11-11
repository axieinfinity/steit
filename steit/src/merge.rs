use std::io::{self, Read};

use iowrap::Eof;

use super::{
    varint::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED},
};

pub trait Merge: WireType {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

    #[inline]
    fn merge_nested(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            let size = Varint::deserialize(reader)?;
            let reader = &mut Eof::new(reader.by_ref().take(size));
            self.merge(reader)
        } else {
            self.merge(reader)
        }
    }
}

impl<T: Varint> Merge for T {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }
}

impl<T: Varint> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = Varint::deserialize(reader)?;
            self.push(item);
        }

        Ok(())
    }
}
