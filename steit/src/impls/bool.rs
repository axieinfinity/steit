use std::io::{self, Read};

use crate::{
    rt::CachedSize,
    ser_v2::{SerializeOmissible, SerializePrimitive},
    wire_format::{HasWireType, WireTypeV2},
};

impl HasWireType for bool {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Varint;
}

impl SerializePrimitive for bool {
    #[inline]
    fn compute_size(&self) -> u32 {
        1
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl SerializeOmissible for bool {
    #[inline]
    fn should_omit(&self) -> bool {
        !*self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util_v2::{assert_serialize, assert_serialize_nested},
    };

    test_case!(serialize_01: assert_serialize; false => &[0]);
    test_case!(serialize_02: assert_serialize; true => &[1]);

    test_case!(serialize_nested_01: assert_serialize_nested; false, None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; true, None => &[1]);
    test_case!(serialize_nested_03: assert_serialize_nested; false, Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; true, Some(10) => &[80, 1]);

    // test_case!(merge_01: assert_merge; false, &[0] => false);
    // test_case!(merge_02: assert_merge; false, &[1] => true);
    // test_case!(merge_03: assert_merge; false, &[128, 128, 128, 128, 128, 128, 128, 128, 128, 2] /* 2^64 */ => true);
    // test_case!(merge_04: assert_merge; true, &[0] => false);
    // test_case!(merge_05: assert_merge; true, &[42] => true);
}
