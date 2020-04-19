use std::io;

use super::{
    rt::CachedSize,
    wire_format::{HasWireType, WireTypeV2},
};

pub trait SerializeV2: HasWireType {
    fn compute_size(&self) -> u32;
    fn cached_size(&self) -> Option<&CachedSize>;
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()>;

    #[inline]
    fn cache_size(&self) -> u32 {
        let size = self.compute_size();

        if let Some(cached_size) = self.cached_size() {
            cached_size.set(size);
        }

        size
    }

    #[inline]
    fn size(&self) -> u32 {
        match self.cached_size() {
            Some(cached_size) => cached_size.get(),
            None => self.compute_size(),
        }
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.cache_size();
        self.serialize_with_cached_size(writer)
    }
}

pub trait SerializeNested: HasWireType {
    fn compute_size_nested(&self, field_number: impl Into<Option<u32>>, is_omissible: bool) -> u32;

    fn serialize_nested(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
        writer: &mut impl io::Write,
    ) -> io::Result<()>;
}

pub trait SerializeOmissible: SerializeV2 {
    #[inline]
    fn should_omit(&self) -> bool {
        self.compute_size() == 0
    }
}

impl<T: SerializeOmissible> SerializeNested for T {
    #[inline]
    fn compute_size_nested(&self, field_number: impl Into<Option<u32>>, is_omissible: bool) -> u32 {
        let field_number = field_number.into();

        if field_number.is_some() && is_omissible && self.should_omit() {
            return 0;
        }

        let mut size = self.cache_size();

        if Self::WIRE_TYPE == WireTypeV2::Sized {
            size += size.cache_size();
        }

        if let Some(field_number) = field_number {
            size += self.tag(field_number).value().cache_size();
        }

        size
    }

    #[inline]
    fn serialize_nested(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        let field_number = field_number.into();

        if field_number.is_some() && is_omissible && self.should_omit() {
            return Ok(());
        }

        if let Some(field_number) = field_number {
            self.tag(field_number)
                .value()
                .serialize_with_cached_size(writer)?;
        }

        if Self::WIRE_TYPE == WireTypeV2::Sized {
            self.size().serialize_with_cached_size(writer)?;
        }

        self.serialize_with_cached_size(writer)
    }
}
