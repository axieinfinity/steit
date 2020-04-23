use std::io;

use super::{
    rt::SizeCache,
    wire_fmt::{HasWireType, WireTypeV2},
};

pub trait SerializeV2: HasWireType {
    fn compute_size(&self) -> u32;
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()>;

    fn size_cache(&self) -> Option<&SizeCache>;

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

    #[inline]
    fn is_omissible(&self) -> bool {
        self.compute_size() == 0
    }

    #[inline]
    fn compute_size_nested_v2(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
    ) -> io::Result<u32> {
        let field_number = field_number.into();

        if field_number.is_some() && is_omissible && self.is_omissible() {
            return Ok(0);
        }

        let mut size = self.cache_size();

        match Self::WIRE_TYPE {
            WireTypeV2::Varint => (),
            WireTypeV2::Sized => size += size.cache_size(),
        }

        if let Some(field_number) = field_number {
            size += self.tag(field_number)?.cache_size();
        }

        Ok(size)
    }

    #[inline]
    fn serialize_nested(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        let field_number = field_number.into();

        if field_number.is_some() && is_omissible && self.is_omissible() {
            return Ok(());
        }

        if let Some(field_number) = field_number {
            self.tag(field_number)?.serialize_cached(writer)?;
        }

        match Self::WIRE_TYPE {
            WireTypeV2::Varint => (),
            WireTypeV2::Sized => self.cached_size().serialize_cached(writer)?,
        }

        self.serialize_cached(writer)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.serialize(&mut bytes).unwrap();
        bytes
    }
}

pub trait SerializePrimitive: PartialEq + Default + HasWireType {
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
    fn size_cache(&self) -> Option<&SizeCache> {
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

    #[inline]
    fn is_omissible(&self) -> bool {
        *self == Self::default()
    }
}
