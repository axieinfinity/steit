use std::io;

use crate::{
    types::Varint,
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Eof, Merge, Serialize,
};

impl<T: Varint> WireType for Vec<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Varint> Serialize for Vec<T> {
    /// Computes serialized size for `Vec<impl Varint>`.
    ///
    /// ```
    /// use steit::Serialize;
    ///
    /// assert_eq!(Vec::<u8>::new().compute_size(), 0);
    /// assert_eq!(vec![0].compute_size(), 1);
    /// assert_eq!(vec![0, 0, 0].compute_size(), 3);
    /// assert_eq!(vec![1337].compute_size(), 2);
    /// ```
    #[inline]
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for item in self {
            size += item.compute_size_nested(None);
        }

        size
    }

    /// Serializes `Vec<impl Varint>`.
    ///
    /// ```
    /// use steit::Serialize;
    ///
    /// let mut bytes = Vec::new();
    /// Vec::<u8>::new().serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[]);
    ///
    /// let mut bytes = Vec::new();
    /// vec![0, 0, 0].serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[0, 0, 0]);
    ///
    /// let mut bytes = Vec::new();
    /// vec![1337].serialize_with_cached_size(&mut bytes).unwrap();
    /// assert_eq!(&bytes, &[242, 20]);
    /// ```
    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            item.serialize_nested_with_cached_size(None, writer)?;
        }

        Ok(())
    }
}

impl<T: Varint> Merge for Vec<T> {
    /// Merges a serialized value with an existing `Vec<impl Varint>`.
    ///
    /// ```
    /// use steit::{Merge, Eof};
    ///
    /// let mut value = Vec::<i32>::new();
    ///
    /// value.merge(&mut Eof::new([1].as_ref())).unwrap();
    /// assert_eq!(&value, &[-1]);
    ///
    /// value.merge(&mut Eof::new([].as_ref())).unwrap();
    /// assert_eq!(&value, &[-1]);
    ///
    /// value.merge(&mut Eof::new([0, 242, 20, 0, 3, 0].as_ref())).unwrap();
    /// assert_eq!(&value, &[-1, 0, 1337, 0, -2, 0]);
    /// ```
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = T::deserialize_nested(reader)?;
            self.push(item);
        }

        Ok(())
    }
}
