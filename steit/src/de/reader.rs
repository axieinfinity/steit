use std::io::{self, Read};

use iowrap::Eof;

use crate::wire_fmt::{self, WireType};

use super::de::Deserialize;

pub struct Reader<R: io::Read> {
    inner: Eof<R>,
}

impl<R: io::Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner: Eof::new(inner),
        }
    }

    pub fn eof(&mut self) -> io::Result<bool> {
        self.inner.eof()
    }

    pub fn nested(&mut self) -> io::Result<Reader<io::Take<&mut Self>>> {
        let size = u64::deserialize(self)?;
        let reader = self.by_ref().take(size);
        Ok(reader.into())
    }

    pub fn read_tag(&mut self) -> io::Result<(u32, WireType)> {
        let value = u32::deserialize(self)?;
        wire_fmt::parse_tag(value)
    }

    pub fn skip_field(&mut self, wire_type: WireType) -> io::Result<()> {
        match wire_type {
            WireType::Varint => {
                u8::deserialize(self)?;
            }

            WireType::Sized => {
                let size = u64::deserialize(self)?;
                let mut buf = Vec::with_capacity(size as usize);
                self.by_ref().take(size).read_to_end(&mut buf)?;
            }
        }

        Ok(())
    }

    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }
}

impl<R: io::Read> io::Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> From<R> for Reader<R> {
    fn from(inner: R) -> Self {
        Self::new(inner)
    }
}
