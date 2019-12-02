use std::io::{self, Read};

use crate::{
    impl_state_for_plain,
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Deserialize, Eof, Merge, Serialize, State,
};

pub trait Varint: Serialize + Deserialize {}

macro_rules! impl_unsigned_varint {
    (u64) => (impl_unsigned_varint!(@impl u64, size_64, i64););
    ($t:ty) => (impl_unsigned_varint!(@impl $t, size_32, i32););

    (@impl $t:ty, $size_fn:ident, $size_t:ty) => {
        impl WireType for $t {
            const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
        }

        impl Serialize for $t {
            #[inline]
            fn compute_size(&self) -> u32 {
                $size_fn(*self as $size_t)
            }

            #[inline]
            fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
                let mut value = *self;

                loop {
                    if value & !0x7f == 0 {
                        return writer.write_all(&[value as u8]);
                    } else {
                        writer.write_all(&[value as u8 & 0x7f | 0x80])?;
                        value >>= 7;
                    }
                }
            }

            #[inline]
            fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
                self.serialize_with_cached_size(writer)
            }

            #[inline]
            fn is_default_nested_with_cached_size(&self) -> bool {
                *self == 0
            }
        }

        impl Merge for $t {
            #[inline]
            fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
                let mut value = 0;

                let mut buf = [0];
                let mut offset = 0;

                loop {
                    reader.read_exact(&mut buf)?;
                    value |= (buf[0] & 0x7f) as $t << offset;

                    if buf[0] & 0x80 == 0 {
                        *self = value;
                        return Ok(());
                    }

                    offset += 7;
                }
            }
        }

        impl Varint for $t {}
    };
}

impl_unsigned_varint!(u8);
impl_unsigned_varint!(u16);
impl_unsigned_varint!(u32);
impl_unsigned_varint!(u64);

macro_rules! impl_signed_varint {
    ($t:ty, $ut:ty) => {
        impl WireType for $t {
            const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
        }

        impl Serialize for $t {
            #[inline]
            fn compute_size(&self) -> u32 {
                (impl_signed_varint!(@encode self, $t) as $ut).compute_size()
            }

            #[inline]
            fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
                (impl_signed_varint!(@encode self, $t) as $ut).serialize_with_cached_size(writer)
            }

            #[inline]
            fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
                self.serialize_with_cached_size(writer)
            }

            #[inline]
            fn is_default_nested_with_cached_size(&self) -> bool {
                *self == 0
            }
        }

        impl Merge for $t {
            #[inline]
            fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
                let encoded = <$ut>::deserialize(reader)? as $t;
                *self = impl_signed_varint!(@decode encoded);
                Ok(())
            }
        }

        impl Varint for $t {}
    };

    // Reference: https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding
    (@encode $value:ident, $t:ty) => {
        ($value << 1) ^ ($value >> ((std::mem::size_of::<$t>() << 3) - 1))
    };

    (@decode $value:ident) => {
        ($value >> 1) ^ -($value & 1)
    };
}

impl_signed_varint!(i8, u8);
impl_signed_varint!(i16, u16);
impl_signed_varint!(i32, u32);
impl_signed_varint!(i64, u64);

impl<T: Varint> State for T {
    impl_state_for_plain!("varint");
}

/// Reference: https://github.com/protocolbuffers/protobuf/blob/342a2d6/java/core/src/main/java/com/google/protobuf/CodedOutputStream.java#L727-L741
#[inline]
fn size_32(value: i32) -> u32 {
    if value & (!0 << 7) == 0 {
        return 1;
    }

    if value & (!0 << 14) == 0 {
        return 2;
    }

    if value & (!0 << 21) == 0 {
        return 3;
    }

    if value & (!0 << 28) == 0 {
        return 4;
    }

    5
}

/// Reference: https://github.com/protocolbuffers/protobuf/blob/342a2d6/java/core/src/main/java/com/google/protobuf/CodedOutputStream.java#L770-L792
#[inline]
fn size_64(mut value: i64) -> u32 {
    // Handle two popular special cases upfront ...
    if value & (!0i64 << 7) == 0 {
        return 1;
    }

    if value < 0 {
        return 10;
    }

    // ... leaving us with 8 remaining, which we can divide and conquer
    let mut size = 2;

    if value & (!0i64 << 35) != 0 {
        size += 4;
        value >>= 28;
    }

    if value & (!0i64 << 21) != 0 {
        size += 2;
        value >>= 14;
    }

    if value & (!0i64 << 14) != 0 {
        size += 1;
    }

    size
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_utils::{
            assert_deserialize, assert_ser_de, assert_serialize, assert_serialize_nested,
        },
    };

    test_case!(encode_zig_zag_01: assert_serialize;  0 => &[0]);
    test_case!(encode_zig_zag_02: assert_serialize; -1 => &[1]);
    test_case!(encode_zig_zag_03: assert_serialize;  1 => &[2]);
    test_case!(encode_zig_zag_04: assert_serialize; -2 => &[3]);
    test_case!(encode_zig_zag_05: assert_serialize;  2 => &[4]);

    test_case!(decode_zig_zag_01: assert_deserialize; &[0] =>  0);
    test_case!(decode_zig_zag_02: assert_deserialize; &[1] => -1);
    test_case!(decode_zig_zag_03: assert_deserialize; &[2] =>  1);
    test_case!(decode_zig_zag_04: assert_deserialize; &[3] => -2);
    test_case!(decode_zig_zag_05: assert_deserialize; &[4] =>  2);

    test_case!(back_and_forth_01: assert_ser_de; -1i8 as u64);
    test_case!(back_and_forth_02: assert_ser_de; !0u64);
    test_case!(back_and_forth_03: assert_ser_de; -1i8 as u32);
    test_case!(back_and_forth_04: assert_ser_de; 1_000_000);
    test_case!(back_and_forth_05: assert_ser_de; 42);

    test_case!(serialize_nested_01: assert_serialize_nested; 0, None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; 1, None => &[2]);
    test_case!(serialize_nested_03: assert_serialize_nested; 0, Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; 1, Some(10) => &[80, 2]);
}
