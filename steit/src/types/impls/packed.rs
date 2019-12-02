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
    #[inline]
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for item in self {
            size += item.compute_size_nested(None);
        }

        size
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            item.serialize_nested_with_cached_size(None, writer)?;
        }

        Ok(())
    }
}

impl<T: Varint> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = T::deserialize_nested(reader)?;
            self.push(item);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size},
    };

    test_case!(size_01: assert_size::<Vec<u8>>; vec![] => 0);
    test_case!(size_02: assert_size; vec![0] => 1);
    test_case!(size_03: assert_size; vec![0, 0, 0] => 3);
    test_case!(size_04: assert_size; vec![1337] => 2);

    test_case!(serialize_01: assert_serialize::<Vec<u8>>; vec![] => &[]);
    test_case!(serialize_02: assert_serialize; vec![0, 0, 0] => &[0, 0, 0]);
    test_case!(serialize_03: assert_serialize; vec![1337] => &[242, 20]);

    test_case!(serialize_nested_01: assert_serialize_nested::<Vec<u8>>; vec![], None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; vec![0], None => &[1, 0]);
    test_case!(serialize_nested_03: assert_serialize_nested::<Vec<u8>>; vec![], Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; vec![0], Some(10) => &[82, 1, 0]);

    test_case!(merge_01: assert_merge::<Vec<u8>>; vec![], &[] => vec![]);
    test_case!(merge_02: assert_merge; vec![], &[1] => vec![-1]);
    test_case!(merge_03: assert_merge; vec![-1], &[] => vec![-1]);
    test_case!(merge_04: assert_merge; vec![-1], &[0, 242, 20, 0, 3, 0] => vec![-1, 0, 1337, 0, -2, 0]);
}
