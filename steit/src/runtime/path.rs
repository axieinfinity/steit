use std::{io, ops};

use iowrap::Eof;

use crate::{
    varint,
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Merge,
};

#[derive(Default)]
pub struct Path {
    path: Vec<u16>,
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl ops::Deref for Path {
    type Target = <Vec<u16> as ops::Deref>::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl WireType for Path {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Merge for Path {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            // TODO: Remove `as Varint` after refactoring `Varint`
            let tag = <u16 as varint::Varint>::deserialize(reader)?;
            self.path.push(tag);
        }

        Ok(())
    }
}
