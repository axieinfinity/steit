use std::io::{self, Read};

use iowrap::Eof;

use crate::wire_fmt::{self, WireTypeV2};

use super::de::DeserializeV2;

pub struct Reader<R: io::Read> {
    inner: Eof<R>,
}

impl<R: io::Read> Reader<R> {
    #[inline]
    pub fn new(inner: R) -> Self {
        Self {
            inner: Eof::new(inner),
        }
    }

    #[inline]
    pub fn eof(&mut self) -> io::Result<bool> {
        self.inner.eof()
    }

    #[inline]
    pub fn nested(&mut self) -> io::Result<Reader<io::Take<&mut Self>>> {
        let size = u64::deserialize_v2(self)?;
        let reader = self.by_ref().take(size);
        Ok(reader.into())
    }

    #[inline]
    pub fn read_tag(&mut self) -> io::Result<(u32, WireTypeV2)> {
        let value = u32::deserialize_v2(self)?;
        wire_fmt::parse_tag(value)
    }

    pub fn skip_field(&mut self, wire_type: WireTypeV2) -> io::Result<()> {
        match wire_type {
            WireTypeV2::Varint => {
                u8::deserialize_v2(self)?;
            }

            WireTypeV2::Sized => {
                let size = u64::deserialize_v2(self)?;
                let mut buf = Vec::with_capacity(size as usize);
                self.by_ref().take(size).read_to_end(&mut buf)?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }
}

impl<R: io::Read> io::Read for Reader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> From<R> for Reader<R> {
    #[inline]
    fn from(inner: R) -> Self {
        Self::new(inner)
    }
}
