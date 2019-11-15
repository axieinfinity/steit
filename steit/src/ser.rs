use std::io;

use super::wire_type::{self, WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT};

pub trait Serialize: WireType {
    fn compute_size(&self) -> u32;
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()>;

    #[inline]
    fn cached_size(&self) -> u32 {
        self.compute_size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.compute_size();
        self.serialize_with_cached_size(writer)
    }

    #[inline]
    fn key(tag: u16) -> u32 {
        wire_type::key(tag, Self::WIRE_TYPE)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.compute_size() == 0
    }

    #[inline]
    fn non_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn size_nested(&self, tag: impl Into<Option<u16>>) -> u32 {
        if self.is_empty() {
            return 0;
        }

        let mut size = self.compute_size();

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            size += size.compute_size();
        }

        if let Some(tag) = tag.into() {
            size += Self::key(tag).compute_size();
        }

        size
    }

    #[inline]
    fn serialize_nested(
        &self,
        tag: impl Into<Option<u16>>,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        if self.is_empty() {
            return Ok(());
        }

        if let Some(tag) = tag.into() {
            Self::key(tag).serialize(writer)?;
        }

        match Self::WIRE_TYPE {
            WIRE_TYPE_VARINT => self.serialize(writer),

            WIRE_TYPE_SIZED => {
                self.compute_size().serialize(writer)?;
                self.serialize(writer)
            }

            wire_type => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected wire type {}", wire_type),
            )),
        }
    }
}
