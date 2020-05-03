use std::io;

use crate::{
    de_v2::{DeserializeV2, Reader},
    meta::{HasTypeMeta, TypeMeta},
    rt::SizeCache,
    ser_v2::SerializeV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl<T> HasWireType for Vec<T> {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl<T: SerializeV2> SerializeV2 for Vec<T> {
    fn compute_size_v2(&self) -> u32 {
        let mut size = 0;

        for item in self {
            size += item.compute_size_nested_v2(None, false).unwrap();
        }

        size
    }

    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for item in self {
            item.serialize_nested(None, false, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        None
    }
}

impl<T: DeserializeV2> DeserializeV2 for Vec<T> {
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let item = T::deserialize_nested_v2(T::WIRE_TYPE, reader)?;
            self.push(item);
        }

        Ok(())
    }
}

impl<T: HasTypeMeta> HasTypeMeta for Vec<T> {
    const TYPE_META: &'static TypeMeta = &TypeMeta::Vec(T::TYPE_REF_META);
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util_v2::{assert_merge, assert_serialize, assert_serialize_nested, assert_size},
    };

    test_case!(size_01: assert_size; Vec::<u8>::new() => 0);
    test_case!(size_02: assert_size; vec![0] => 1);
    test_case!(size_03: assert_size; vec![0, 0, 0] => 3);
    test_case!(size_04: assert_size; vec![1337] => 2);

    test_case!(serialize_01: assert_serialize; Vec::<u8>::new() => &[]);
    test_case!(serialize_02: assert_serialize; vec![0, 0, 0] => &[0, 0, 0]);
    test_case!(serialize_03: assert_serialize; vec![1337] => &[242, 20]);

    test_case!(serialize_nested_01: assert_serialize_nested; Vec::<u8>::new(), None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; vec![0], None => &[1, 0]);
    test_case!(serialize_nested_03: assert_serialize_nested; Vec::<u8>::new(), 10 => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; vec![0], 10 => &[82, 1, 0]);

    test_case!(merge_01: assert_merge; Vec::<u8>::new(), &[] => vec![]);
    test_case!(merge_02: assert_merge; vec![], &[1] => vec![-1]);
    test_case!(merge_03: assert_merge; vec![-1], &[] => vec![-1]);
    test_case!(merge_04: assert_merge; vec![-1], &[0, 242, 20, 0, 3, 0] => vec![-1, 0, 1337, 0, -2, 0]);
}
