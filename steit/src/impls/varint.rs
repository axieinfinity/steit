use std::io::{self, Read};

use crate::{
    de_v2::{DeserializeV2, Reader},
    impl_meta_primitive, impl_state_primitive,
    ser_v2::SerializePrimitive,
    wire_fmt::{HasWireType, WireTypeV2},
};

macro_rules! impl_unsigned_varint {
    (u64, "UInt64") => {
        impl_unsigned_varint!(@impl u64, size_64, i64, "UInt64");
    };

    ($type:ty, $csharp_name:literal) => {
        impl_unsigned_varint!(@impl $type, size_32, i32, $csharp_name);
    };

    (@impl $type:ty, $size_fn:ident, $size_type:ty, $csharp_name:literal) => {
        impl HasWireType for $type {
            const WIRE_TYPE: WireTypeV2 = WireTypeV2::Varint;
        }

        impl SerializePrimitive for $type {
            #[inline]
            fn compute_size(&self) -> u32 {
                $size_fn(*self as $size_type)
            }

            fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
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
        }

        impl DeserializeV2 for $type {
            fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                let mut value = 0;

                let mut buf = [0];
                let mut offset = 0;

                loop {
                    reader.read_exact(&mut buf)?;
                    value |= (buf[0] & 0x7f) as $type << offset;

                    if buf[0] & 0x80 == 0 {
                        *self = value;
                        return Ok(());
                    }

                    offset += 7;
                }
            }
        }

        impl_state_primitive!($type);
        impl_meta_primitive!($type, $csharp_name);
    };
}

impl_unsigned_varint!(u8, "Byte");
impl_unsigned_varint!(u16, "UInt16");
impl_unsigned_varint!(u32, "UInt32");
impl_unsigned_varint!(u64, "UInt64");

macro_rules! impl_signed_varint {
    ($type:ty, $unsigned_type:ty, $csharp_name:literal) => {
        impl HasWireType for $type {
            const WIRE_TYPE: WireTypeV2 = WireTypeV2::Varint;
        }

        impl SerializePrimitive for $type {
            #[inline]
            fn compute_size(&self) -> u32 {
                (impl_signed_varint!(@encode self, $type) as $unsigned_type).compute_size()
            }

            #[inline]
            fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
                (impl_signed_varint!(@encode self, $type) as $unsigned_type).serialize(writer)
            }
        }

        impl DeserializeV2 for $type {
            #[inline]
            fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                let encoded = <$unsigned_type>::deserialize_v2(reader)? as $type;
                *self = impl_signed_varint!(@decode encoded);
                Ok(())
            }
        }

        impl_state_primitive!($type);
        impl_meta_primitive!($type, $csharp_name);
    };

    // More about Zigzag encoding can be found at:
    // https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding

    (@encode $value:ident, $type:ty) => {
        ($value << 1) ^ ($value >> ((std::mem::size_of::<$type>() << 3) - 1))
    };

    (@decode $value:ident) => {
        ($value >> 1) ^ -($value & 1)
    };
}

impl_signed_varint!(i8, u8, "SByte");
impl_signed_varint!(i16, u16, "Int16");
impl_signed_varint!(i32, u32, "Int32");
impl_signed_varint!(i64, u64, "Int64");

/// Gets varint size in bytes of a 32-bit integer.
///
/// This references [`CodedOutputStream`] implementation in [Java]
/// from the official [Protocol Buffers] [repository].
///
/// [Java]: https://en.wikipedia.org/wiki/Java_(programming_language)
/// [Protocol Buffers]: https://developers.google.com/protocol-buffers
/// [repository]: https://github.com/protocolbuffers/protobuf
/// [`CodedOutputStream`]: https://github.com/protocolbuffers/protobuf/blob/342a2d6/java/core/src/main/java/com/google/protobuf/CodedOutputStream.java#L727-L741
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

/// Gets varint size in bytes of a 64-bit integer.
///
/// This references [`CodedOutputStream`] implementation in [Java]
/// from the official [Protocol Buffers] [repository].
///
/// [Java]: https://en.wikipedia.org/wiki/Java_(programming_language)
/// [Protocol Buffers]: https://developers.google.com/protocol-buffers
/// [repository]: https://github.com/protocolbuffers/protobuf
/// [`CodedOutputStream`]: https://github.com/protocolbuffers/protobuf/blob/342a2d6/java/core/src/main/java/com/google/protobuf/CodedOutputStream.java#L770-L792
#[inline]
fn size_64(mut value: i64) -> u32 {
    // Handle two popular special cases upfront …
    if value & (!0i64 << 7) == 0 {
        return 1;
    }

    if value < 0 {
        return 10;
    }

    // … leaving us with 8 remaining, which we can divide and conquer
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
        test_util_v2::{
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
