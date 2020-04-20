use std::{
    io::{self, Read},
    ops::{Deref, DerefMut},
};

use iowrap::Eof;

use super::{de_v2::DeserializeV2, wire_format::WireTypeV2};

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

impl<R: io::Read> Deref for Reader<R> {
    type Target = R;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.get_ref()
    }
}

impl<R: io::Read> DerefMut for Reader<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.get_mut()
    }
}

impl<R: io::Read> From<R> for Reader<R> {
    #[inline]
    fn from(inner: R) -> Self {
        Self::new(inner)
    }
}
