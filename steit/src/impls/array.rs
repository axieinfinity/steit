use std::io::{self, Read};

use crate::{
    de::{Deserialize, Reader},
    meta::{FieldTypeMeta, HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::SizeCache,
    ser::Serialize,
    wire_fmt::{HasWireType, WireType},
};

macro_rules! impl_array {
    ($len:literal) => {
        impl<T> HasWireType for [T; $len] {
            const WIRE_TYPE: WireType = WireType::Sized;
        }

        impl<T: Serialize> Serialize for [T; $len] {
            fn compute_size(&self) -> u32 {
                let mut size = 0;

                for item in self {
                    size += item.compute_size_nested(None, false).unwrap();
                }

                size
            }

            fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
                for item in self {
                    item.serialize_nested(None, false, writer)?;
                }

                Ok(())
            }

            fn size_cache(&self) -> Option<&SizeCache> {
                None
            }
        }

        impl<T: Deserialize> Deserialize for [T; $len] {
            fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                let mut index = 0;

                while index < $len && !reader.eof()? {
                    let item = T::deserialize_nested(T::WIRE_TYPE, reader)?;
                    self[index] = item;
                    index += 1;
                }

                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes)?;

                Ok(())
            }
        }

        impl<T: HasMeta> HasMeta for [T; $len] {
            const NAME: &'static NameMeta = &NameMeta {
                rust: "Vec",
                csharp: Some("Vector"),
            };

            const TYPE: &'static TypeMeta =
                &TypeMeta::Ref(Self::NAME, &[FieldTypeMeta::Type(T::TYPE)]);

            const LINK: &'static MetaLink = &MetaLink {
                r#type: Self::TYPE,
                msg: None,
                links: || &[T::LINK],
            };
        }
    };
}

impl_array!(1);
impl_array!(2);
impl_array!(3);
impl_array!(4);
impl_array!(5);
impl_array!(6);
impl_array!(7);

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size},
    };

    test_case!(size_01: assert_size; [0, 0] => 2);
    test_case!(size_02: assert_size; [1337, 1337, 1337, 1337] => 8);

    test_case!(serialize_01: assert_serialize; [0] => &[0]);
    test_case!(serialize_02: assert_serialize; [1337, 1338, 42] => &[242, 20, 244, 20, 84]);

    test_case!(serialize_nested_01: assert_serialize_nested; [0, 0, 0], None => &[3, 0, 0, 0]);
    test_case!(serialize_nested_02: assert_serialize_nested; [42, 42, 42], 2 => &[18, 3, 84, 84, 84]);

    test_case!(merge_01: assert_merge; [1, 2, 3], &[1, 2, 3] => [-1, 1, -2]);
    test_case!(merge_02: assert_merge; [1, 2, 3], &[1] => [-1, 2, 3]);
}
