use std::io;

use super::{
    rt::CachedSize,
    wire_format::{HasWireType, WireTypeV2},
};

pub trait SerializeV2: HasWireType {
    fn compute_size(&self) -> u32;
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()>;

    fn size_cache(&self) -> Option<&CachedSize>;

    #[inline]
    fn cache_size(&self) -> u32 {
        let size = self.compute_size();

        if let Some(size_cache) = self.size_cache() {
            size_cache.set(size);
        }

        size
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        match self.size_cache() {
            Some(size_cache) => size_cache.get(),
            None => self.compute_size(),
        }
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.cache_size();
        self.serialize_cached(writer)
    }
}

pub trait SerializePrimitive: HasWireType {
    fn compute_size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;
}

impl<T: SerializePrimitive> SerializeV2 for T {
    #[inline]
    fn compute_size(&self) -> u32 {
        SerializePrimitive::compute_size(self)
    }

    #[inline]
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.serialize(writer)
    }

    #[inline]
    fn size_cache(&self) -> Option<&CachedSize> {
        None
    }

    #[inline]
    fn cache_size(&self) -> u32 {
        self.compute_size()
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.compute_size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        SerializePrimitive::serialize(self, writer)
    }
}

pub trait SerializeNested: HasWireType {
    fn compute_size_nested_v2(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
    ) -> u32;

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
    fn compute_size_nested_v2(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
    ) -> u32 {
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
            self.tag(field_number).value().serialize_cached(writer)?;
        }

        if Self::WIRE_TYPE == WireTypeV2::Sized {
            self.cached_size().serialize_cached(writer)?;
        }

        self.serialize_cached(writer)
    }
}
