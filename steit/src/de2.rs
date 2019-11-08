use std::io::{self, Read};

use iowrap::Eof;

use crate::{
    varint::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED, WIRE_TYPE_VARINT},
    Runtime2,
};

pub trait Deserialize: Default + WireType {
    fn set_runtime(&mut self, runtime: Runtime2) -> &mut Self;
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()>;

    #[inline]
    fn with_runtime(runtime: Runtime2) -> Self {
        // We use `Self::` since surprisingly `Default::` leaves us with an unknown type.
        let mut state = Self::default();
        state.set_runtime(runtime);
        state
    }

    #[inline]
    fn merge_nested(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            // TODO: Remove `as Deserialize` after refactoring `Varint`
            let size = <u64 as Deserialize>::deserialize(reader)?;
            let reader = &mut Eof::new(reader.by_ref().take(size));
            self.merge(reader)
        } else {
            self.merge(reader)
        }
    }

    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        // We use `Self::` because of the same reason as in `with_runtime`.
        let mut value = Self::default();
        value.merge(reader)?;
        Ok(value)
    }
}

impl<T: Default + Varint + WireType> Deserialize for T {
    #[inline]
    fn set_runtime(&mut self, runtime: Runtime2) -> &mut Self {
        self
    }

    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Varint::deserialize(reader)?;
        Ok(())
    }

    #[inline]
    fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Self> {
        Varint::deserialize(reader)
    }
}

pub fn exhaust_nested(tag: u16, wire_type: u8, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
    match wire_type {
        WIRE_TYPE_VARINT => {
            // TODO: Remove `as Deserialize` after refactoring `Varint`
            <u8 as Deserialize>::deserialize(reader)?;
        }

        WIRE_TYPE_SIZED => {
            // TODO: Remove `as Deserialize` after refactoring `Varint`
            let size = <u64 as Deserialize>::deserialize(reader)?;
            let mut buf = Vec::new();
            reader.by_ref().take(size).read_to_end(&mut buf)?;
        }

        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected tag {} or wire type {}", tag, wire_type),
            ))
        }
    }

    Ok(())
}
