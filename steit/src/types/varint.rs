use std::io::{self, Read};

use crate::{
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Deserialize, Eof, Merge, ReplayKind, Runtime, Serialize, State,
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
                if *self != Self::default() {
                    $size_fn(*self as $size_t)
                } else {
                    0
                }
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
                if *self != Self::default() {
                    (impl_signed_varint!(@encode self, $t) as $ut).compute_size()
                } else {
                    0
                }
            }

            #[inline]
            fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
                (impl_signed_varint!(@encode self, $t) as $ut).serialize_with_cached_size(writer)
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

    // ZigZag encoding: https://bit.ly/2Pl9Gq8
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
    #[inline]
    fn with_runtime(_runtime: Runtime) -> Self {
        Self::default()
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        panic!("cannot get a `Runtime` from a varint")
    }

    #[inline]
    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()> {
        if let Some(tag) = path.next() {
            let mut s = format!("{}", tag);

            for tag in path {
                s.push_str(&format!(", {}", tag));
            }

            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected end-of-path but still got [{}] remaining", s),
            ))
        } else {
            match kind {
                ReplayKind::Update => self.handle_update(reader),

                ReplayKind::Add | ReplayKind::Remove => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`add` and `remove` are not supported on varints",
                )),
            }
        }
    }

    #[inline]
    fn handle_update(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Self::deserialize(reader)?;
        Ok(())
    }
}

// Reference: https://bit.ly/2BJbkd5
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

// Reference: https://bit.ly/2MPq54D
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
mod test {
    use std::fmt;

    use crate::{test_case, Eof};

    use super::Varint;

    fn encode(value: impl Varint) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(10);
        value.serialize(&mut bytes).unwrap();
        bytes
    }

    fn assert_encode(value: impl Varint, expected_bytes: &[u8]) {
        assert_eq!(&*encode(value), expected_bytes);
    }

    test_case!(encode_zig_zag_01: assert_encode;  0 => &[0]);
    test_case!(encode_zig_zag_02: assert_encode; -1 => &[1]);
    test_case!(encode_zig_zag_03: assert_encode;  1 => &[2]);
    test_case!(encode_zig_zag_04: assert_encode; -2 => &[3]);
    test_case!(encode_zig_zag_05: assert_encode;  2 => &[4]);

    fn decode<T: Varint>(bytes: &[u8]) -> T {
        T::deserialize(&mut Eof::new(bytes)).unwrap()
    }

    fn assert_decode<T: PartialEq + fmt::Debug + Varint>(bytes: &[u8], expected_value: T) {
        assert_eq!(decode::<T>(bytes), expected_value);
    }

    test_case!(decode_zig_zag_01: assert_decode; &[0] =>  0);
    test_case!(decode_zig_zag_02: assert_decode; &[1] => -1);
    test_case!(decode_zig_zag_03: assert_decode; &[2] =>  1);
    test_case!(decode_zig_zag_04: assert_decode; &[3] => -2);
    test_case!(decode_zig_zag_05: assert_decode; &[4] =>  2);

    fn assert_back_and_forth<T: Copy + PartialEq + fmt::Debug + Varint>(value: T) {
        assert_eq!(decode::<T>(&*encode(value)), value);
    }

    test_case!(back_and_forth_01: assert_back_and_forth; -1i8 as u64);
    test_case!(back_and_forth_02: assert_back_and_forth; !0u64);
    test_case!(back_and_forth_03: assert_back_and_forth; -1i8 as u32);
    test_case!(back_and_forth_04: assert_back_and_forth; 1_000_000);
    test_case!(back_and_forth_05: assert_back_and_forth; 42);
}
