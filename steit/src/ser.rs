use std::io;

use super::{
    rt::SizeCache,
    wire_fmt::{HasWireType, WireType},
};

pub trait Serialize: HasWireType {
    fn compute_size(&self) -> u32;
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()>;

    fn size_cache(&self) -> Option<&SizeCache>;

    fn cache_size(&self) -> u32 {
        let size = self.compute_size();

        if let Some(size_cache) = self.size_cache() {
            size_cache.set(size);
        }

        size
    }

    fn cached_size(&self) -> u32 {
        match self.size_cache() {
            Some(size_cache) => size_cache.get(),
            None => self.compute_size(),
        }
    }

    fn steit_serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.cache_size();
        self.serialize_cached(writer)
    }

    fn is_omissible(&self, size_hint: Option<u32>) -> bool {
        match size_hint {
            Some(size) => size == 0,
            None => self.cached_size() == 0,
        }
    }

    fn compute_size_nested(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
    ) -> io::Result<u32> {
        let field_number = field_number.into();
        let mut size = self.cache_size();

        if field_number.is_some() && is_omissible && self.is_omissible(Some(size)) {
            return Ok(0);
        }

        match Self::WIRE_TYPE {
            WireType::Varint => (),
            WireType::Sized => size += size.cache_size(),
        }

        if let Some(field_number) = field_number {
            size += self.tag(field_number)?.cache_size();
        }

        Ok(size)
    }

    fn serialize_nested(
        &self,
        field_number: impl Into<Option<u32>>,
        is_omissible: bool,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        let field_number = field_number.into();

        if field_number.is_some() && is_omissible && self.is_omissible(None) {
            return Ok(());
        }

        if let Some(field_number) = field_number {
            self.tag(field_number)?.serialize_cached(writer)?;
        }

        match Self::WIRE_TYPE {
            WireType::Varint => (),
            WireType::Sized => self.cached_size().serialize_cached(writer)?,
        }

        self.serialize_cached(writer)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.steit_serialize(&mut bytes).unwrap();
        bytes
    }
}
