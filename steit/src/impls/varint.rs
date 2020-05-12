macro_rules! impl_unsigned_varint {
    (u64, $dummy:ident, "UInt64") => {
        impl_unsigned_varint!(@impl u64, $dummy, size_64, i64, "UInt64");
    };

    ($type:ty, $dummy:ident, $csharp_name:literal) => {
        impl_unsigned_varint!(@impl $type, $dummy, size_32, i32, $csharp_name);
    };

    (@impl $type:ty, $dummy:ident, $size_fn:ident, $size_type:ty, $csharp_name:literal) => {
        const $dummy: () = {
            impl $crate::wire_fmt::HasWireType for $type {
                const WIRE_TYPE: $crate::wire_fmt::WireType =
                    $crate::wire_fmt::WireType::Varint;
            }

            #[inline]
            fn compute_size(value: &$type) -> u32 {
                $size_fn(*value as $size_type)
            }

            #[inline]
            fn serialize(
                value: &$type,
                writer: &mut impl ::std::io::Write,
            ) -> ::std::io::Result<()> {
                let mut value = *value;

                loop {
                    if value & !0x7f == 0 {
                        return writer.write_all(&[value as u8]);
                    } else {
                        writer.write_all(&[value as u8 & 0x7f | 0x80])?;
                        value >>= 7;
                    }
                }
            }

            $crate::impl_serialize_primitive!($type, compute_size, serialize);

            impl $crate::de::Deserialize for $type {
                fn merge(
                    &mut self,
                    reader: &mut $crate::de::Reader<impl ::std::io::Read>,
                ) -> ::std::io::Result<()> {
                    use ::std::io::Read;

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

            $crate::impl_state_primitive!($type);
            $crate::impl_meta_primitive!($type, $csharp_name);
        };
    };
}

impl_unsigned_varint!(u8, _U8_IMPLS, "Byte");
impl_unsigned_varint!(u16, _U16_IMPLS, "UInt16");
impl_unsigned_varint!(u32, _U32_IMPLS, "UInt32");
impl_unsigned_varint!(u64, _U64_IMPLS, "UInt64");

macro_rules! impl_signed_varint {
    ($type:ty, $dummy:ident, $unsigned_type:ty, $csharp_name:literal) => {
        const $dummy: () = {
            impl $crate::wire_fmt::HasWireType for $type {
                const WIRE_TYPE: $crate::wire_fmt::WireType = $crate::wire_fmt::WireType::Varint;
            }

            // More about Zigzag encoding can be found at:
            // https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding

            #[inline]
            fn encode(value: $type) -> $type {
                (value << 1) ^ (value >> ((std::mem::size_of::<$type>() << 3) - 1))
            }

            #[inline]
            fn decode(value: $type) -> $type {
                (value >> 1) ^ -(value & 1)
            }

            #[inline]
            fn compute_size(value: &$type) -> u32 {
                use $crate::ser::Serialize;
                (encode(*value) as $unsigned_type).compute_size()
            }

            #[inline]
            fn serialize(
                value: &$type,
                writer: &mut impl ::std::io::Write,
            ) -> ::std::io::Result<()> {
                use $crate::ser::Serialize;
                (encode(*value) as $unsigned_type).serialize(writer)
            }

            $crate::impl_serialize_primitive!($type, compute_size, serialize);

            impl $crate::de::Deserialize for $type {
                #[inline]
                fn merge(
                    &mut self,
                    reader: &mut $crate::de::Reader<impl ::std::io::Read>,
                ) -> ::std::io::Result<()> {
                    let encoded = <$unsigned_type>::deserialize(reader)? as $type;
                    *self = decode(encoded);
                    Ok(())
                }
            }

            $crate::impl_state_primitive!($type);
            $crate::impl_meta_primitive!($type, $csharp_name);
        };
    };
}

impl_signed_varint!(i8, _I8_IMPLS, u8, "SByte");
impl_signed_varint!(i16, _I16_IMPLS, u16, "Int16");
impl_signed_varint!(i32, _I32_IMPLS, u32, "Int32");
impl_signed_varint!(i64, _I64_IMPLS, u64, "Int64");

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
        test_util::{assert_deserialize, assert_ser_de, assert_serialize, assert_serialize_nested},
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
