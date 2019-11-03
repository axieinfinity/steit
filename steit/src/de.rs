use std::io;

use crate::varint::Varint;

pub trait Deserialize: Sized {
    fn deserialize(&mut self, reader: &mut impl io::Read) -> io::Result<()>;
}

impl<T: Varint> Deserialize for T {
    fn deserialize(&mut self, reader: &mut impl io::Read) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }
}

impl<T: Varint> Deserialize for Vec<T> {
    fn deserialize(&mut self, reader: &mut impl io::Read) -> io::Result<()> {
        use io::Read;

        let size = Varint::deserialize(reader)?;
        let reader = &mut iowrap::Eof::new(reader.by_ref().take(size));

        self.clear();

        while !reader.eof()? {
            let item = Varint::deserialize(reader)?;
            self.push(item);
        }

        Ok(())
    }
}
