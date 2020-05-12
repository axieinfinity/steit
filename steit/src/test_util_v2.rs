use std::fmt;

use super::{
    de_v2::{DeserializeV2, Reader},
    rt::{RuntimeV2, SizeCache},
    ser_v2::SerializeV2,
    state_v2::StateV2,
    steit_derive,
};

#[macro_export]
macro_rules! test_case {
    ($name:ident : $assert:expr ; $($args:expr),+) => {
        #[test]
        fn $name() {
            $assert($($args),+);
        }
    };

    ($name:ident : $assert:expr ; $($input:expr),+ => $($output:expr),+) => {
        test_case!($name : $assert ; $($input),+, $($output),+);
    };
}

#[steit_derive(Debug, Serialize, Deserialize)]
#[steit(steit_owned)]
pub struct Foo(#[steit(tag = 0)] pub i32, #[steit(tag = 1)] pub i32);

impl Foo {
    #[inline]
    pub fn new(f0: i32, f1: i32) -> Self {
        Self {
            0: f0,
            1: f1,
            2: SizeCache::new(),
        }
    }
}

#[steit_derive(Debug, State)]
#[steit(steit_owned)]
pub struct Point {
    #[steit(tag = 0)]
    pub x: i32,
    #[steit(tag = 1)]
    pub y: i32,
    #[steit(tag = 2)]
    pub z: i32,
}

impl Point {
    #[inline]
    pub fn new(runtime: RuntimeV2, x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            size_cache: SizeCache::new(),
            runtime,
        }
    }
}

pub fn assert_size(value: impl SerializeV2, size: u32) {
    assert_eq!(value.compute_size_v2(), size);
}

pub fn serialize(value: impl SerializeV2) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.serialize_v2(&mut bytes).unwrap();
    bytes
}

pub fn assert_serialize(value: impl SerializeV2, bytes: &[u8]) {
    assert_eq!(&*serialize(value), bytes);
}

pub fn serialize_nested(value: impl SerializeV2, tag: impl Into<Option<u32>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.cache_size();
    value.serialize_nested(tag, true, &mut bytes).unwrap();
    bytes
}

pub fn assert_serialize_nested(value: impl SerializeV2, tag: impl Into<Option<u32>>, bytes: &[u8]) {
    assert_eq!(&*serialize_nested(value, tag), bytes);
}

pub fn merge<T: DeserializeV2>(value: &mut T, bytes: &[u8]) {
    value.merge_v2(&mut Reader::new(bytes)).unwrap();
}

pub fn assert_merge<T: PartialEq + fmt::Debug + DeserializeV2>(
    mut value: T,
    bytes: &[u8],
    expected_value: T,
) {
    merge(&mut value, bytes);
    assert_eq!(value, expected_value);
}

pub fn deserialize<T: DeserializeV2>(bytes: &[u8]) -> T {
    T::deserialize_v2(&mut Reader::new(bytes)).unwrap()
}

pub fn assert_deserialize<T: PartialEq + fmt::Debug + DeserializeV2>(bytes: &[u8], value: T) {
    assert_eq!(deserialize::<T>(bytes), value);
}

pub fn assert_ser_de<T: Clone + PartialEq + fmt::Debug + SerializeV2 + DeserializeV2>(value: T) {
    assert_eq!(deserialize::<T>(&*serialize(value.clone())), value);
}

pub fn replay<T: StateV2>(value: &mut T, bytes: &[u8]) {
    value.replay(&mut Reader::new(bytes)).unwrap();
}
