use std::io;

use crate::{
    varint,
    wire_type::{self, WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
};

pub trait Serialize: WireType {
    fn size(&self) -> u32;
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()>;

    #[inline]
    fn key(tag: u16) -> u32 {
        wire_type::key(tag, Self::WIRE_TYPE)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
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

        let mut size = self.size();

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            size += size.size();
        }

        if let Some(tag) = tag.into() {
            size += Self::key(tag).size();
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
                self.size().serialize(writer)?;
                self.serialize(writer)
            }

            wire_type => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected wire type {}", wire_type),
            )),
        }
    }
}

impl<T: Default + Eq + varint::Varint + WireType> Serialize for T {
    #[inline]
    fn size(&self) -> u32 {
        varint::Varint::size(self) as u32
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        varint::Varint::serialize(self, writer)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}
