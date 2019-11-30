use std::io;

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Deserialize, Eof, Merge, Serialize,
};

impl<T> WireType for Option<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Serialize> Serialize for Option<T> {
    /// Computes serialized size for `Option`.
    ///
    /// ```
    /// use steit::Serialize;
    ///
    /// assert_eq!(None::<u8>.compute_size(), 0);
    /// assert_eq!(Some(0).compute_size(), 1);
    /// assert_eq!(Some(1).compute_size(), 1);
    /// assert_eq!(Some(1337).compute_size(), 2);
    /// ```
    #[inline]
    fn compute_size(&self) -> u32 {
        match self {
            Some(value) => value.compute_size_nested(None),
            None => 0,
        }
    }

    /// Serializes `Option`.
    ///
    /// ```
    /// use steit::Serialize;
    ///
    /// let mut bytes = Vec::new();
    /// None::<u8>.serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[]);
    ///
    /// let mut bytes = Vec::new();
    /// Some(0).serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[0]);
    ///
    /// let mut bytes = Vec::new();
    /// Some(1337).serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[242, 20]);
    /// ```
    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Some(value) => value.serialize_nested_with_cached_size(None, writer),
            None => Ok(()),
        }
    }
}

impl<T: Deserialize> Merge for Option<T> {
    /// Merges a serialized value with an existing `Option`.
    ///
    /// ```
    /// use steit::{Merge, Eof};
    ///
    /// let mut value = Some(1);
    ///
    /// value.merge(&mut Eof::new([].as_ref())).unwrap();
    /// assert_eq!(value, None);
    ///
    /// value.merge(&mut Eof::new([0].as_ref())).unwrap();
    /// assert_eq!(value, Some(0));
    ///
    /// value.merge(&mut Eof::new([0, 242, 20].as_ref())).unwrap();
    /// assert_eq!(value, Some(1337));
    /// ```
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if !reader.eof()? {
            while !reader.eof()? {
                let value = T::deserialize_nested(reader)?;
                *self = Some(value);
            }
        } else {
            *self = None;
        }

        Ok(())
    }
}
