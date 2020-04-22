use std::{collections::HashMap, hash::Hash, io};

use crate::{
    de_v2::{DeserializeV2, Reader},
    rt::SizeCache,
    ser_v2::SerializeV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl<K, V> HasWireType for HashMap<K, V> {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl<K: SerializeV2, V: SerializeV2> SerializeV2 for HashMap<K, V> {
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for (key, value) in self {
            size += key.compute_size_nested_v2(None, false).unwrap();
            size += value.compute_size_nested_v2(None, false).unwrap();
        }

        size
    }

    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for (key, value) in self {
            key.serialize_nested(None, false, writer)?;
            value.serialize_nested(None, false, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        None
    }
}

impl<K: Eq + Hash + DeserializeV2, V: DeserializeV2> DeserializeV2 for HashMap<K, V> {
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let key = K::deserialize_nested_v2(K::WIRE_TYPE, reader)?;
            let value = V::deserialize_nested_v2(V::WIRE_TYPE, reader)?;
            self.insert(key, value);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        test_case,
        test_util_v2::{
            assert_merge, assert_ser_de, assert_serialize, assert_serialize_nested, assert_size,
        },
    };

    macro_rules! hash_map {
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }};
    }

    test_case!(size_01: assert_size; HashMap::<u8, i8>::new() => 0);
    test_case!(size_02: assert_size; hash_map!(0 => 1) => 2);
    test_case!(size_03: assert_size; hash_map!(0 => 1, 1 => 2, 2 => 3) => 6);
    test_case!(size_04: assert_size; hash_map!(1337 => 1337) => 4);

    test_case!(serialize_01: assert_serialize; HashMap::<u8, i8>::new() => &[]);
    test_case!(serialize_02: assert_serialize; hash_map!(1337u16 => 1337) => &[185, 10, 242, 20]);

    test_case!(serialize_nested_01: assert_serialize_nested; HashMap::<u8, i8>::new(), None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; hash_map!(0 => 1), None => &[2, 0, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; HashMap::<u8, i8>::new(), 10 => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; hash_map!(0 => 1), 10 => &[82, 2, 0, 2]);

    test_case!(merge_01: assert_merge; HashMap::<u8, i8>::new(), &[] => HashMap::new());
    test_case!(merge_02: assert_merge; HashMap::<u8, i8>::new(), &[2, 1] => hash_map!(2 => -1));
    test_case!(merge_03: assert_merge; hash_map!(2 => -1), &[] => hash_map!(2 => -1));

    test_case!(back_and_forth_01: assert_ser_de; hash_map!(0 => 1, 1 => 2, 2 => 3));
    test_case!(back_and_forth_02: assert_ser_de; hash_map!(0 => -1, -1 => 2, 2 => -3));
    test_case!(back_and_forth_03: assert_ser_de; hash_map!(-1337 => 1337, -1_000_000 => 1_000_000));
    test_case!(back_and_forth_04: assert_ser_de; hash_map!(42 => 42));
}
