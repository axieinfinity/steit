use std::io::{self, Read};

use crate::{
    types::Varint,
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Eof, Merge, Serialize,
};

impl WireType for bool {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
}

impl Serialize for bool {
    #[inline]
    fn compute_size(&self) -> u32 {
        1
    }

    /// Serializes `bool`.
    ///
    /// ```
    /// use steit::Serialize;
    ///
    /// let mut bytes = Vec::new();
    /// false.serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[0]);
    ///
    /// let mut bytes = Vec::new();
    /// true.serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[1]);
    /// ```
    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.serialize_with_cached_size(writer)
    }
}

impl Merge for bool {
    /// Merges a serialized value with an existing `bool`.
    ///
    /// ```
    /// use steit::{Merge, Eof};
    ///
    /// let mut value = false;
    ///
    /// value.merge(&mut Eof::new([128, 2].as_ref())).unwrap();
    /// assert_eq!(value, true);
    ///
    /// value.merge(&mut Eof::new([0].as_ref())).unwrap();
    /// assert_eq!(value, false);
    /// ```
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        let mut value = false;
        let mut buf = [0];

        loop {
            reader.read_exact(&mut buf)?;
            value |= buf[0] & 0x7f != 0;

            if buf[0] & 0x80 == 0 {
                *self = value;
                return Ok(());
            }
        }
    }
}

impl Varint for bool {}
