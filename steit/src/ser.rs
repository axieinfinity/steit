use std::io;

use super::wire_type::{self, WireType, WIRE_TYPE_SIZED};

pub trait Serialize: WireType {
    fn compute_size(&self) -> u32;
    fn cached_size(&self) -> u32;
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()>;

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
    fn compute_size_nested(&self, tag: impl Into<Option<u16>>) -> u32 {
        let mut size = self.compute_size();

        if size == 0 {
            return 0;
        }

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            size += size.compute_size();
        }

        if let Some(tag) = tag.into() {
            size += Self::key(tag).compute_size();
        }

        size
    }

    #[inline]
    fn serialize_nested_with_cached_size(
        &self,
        tag: impl Into<Option<u16>>,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        if self.cached_size() == 0 {
            return Ok(());
        }

        if let Some(tag) = tag.into() {
            Self::key(tag).serialize_with_cached_size(writer)?;
        }

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            self.cached_size().serialize_with_cached_size(writer)?;
        }

        self.serialize_with_cached_size(writer)
    }
}
