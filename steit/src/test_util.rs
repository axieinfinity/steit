use std::fmt;

use super::{
    de::{Deserialize, Reader},
    rt::{Runtime, SizeCache},
    ser::Serialize,
    state::State,
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

#[steit_derive(PartialEq, Debug, Serialize, Deserialize)]
#[steit(steit_owned, ctor_prefix = "empty")]
pub struct Foo(#[steit(tag = 0)] pub i32, #[steit(tag = 1)] pub i32);

impl Foo {
    pub fn new(f0: i32, f1: i32) -> Self {
        Self {
            0: f0,
            1: f1,
            2: SizeCache::new(),
        }
    }
}

#[steit_derive(PartialEq, Debug, State)]
#[steit(steit_owned, ctor_prefix = "empty")]
pub struct Point {
    #[steit(tag = 0)]
    pub x: i32,
    #[steit(tag = 1)]
    pub y: i32,
    #[steit(tag = 2)]
    pub z: i32,
}

impl Point {
    pub fn new(runtime: Runtime, x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            size_cache: SizeCache::new(),
            runtime,
        }
    }
}

pub fn assert_size(value: impl Serialize, size: u32) {
    assert_eq!(value.compute_size(), size);
}

pub fn serialize(value: impl Serialize) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.serialize(&mut bytes).unwrap();
    bytes
}

pub fn assert_serialize(value: impl Serialize, bytes: &[u8]) {
    assert_eq!(&*serialize(value), bytes);
}

pub fn serialize_nested(value: impl Serialize, tag: impl Into<Option<u32>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.cache_size();
    value.serialize_nested(tag, true, &mut bytes).unwrap();
    bytes
}

pub fn assert_serialize_nested(value: impl Serialize, tag: impl Into<Option<u32>>, bytes: &[u8]) {
    assert_eq!(&*serialize_nested(value, tag), bytes);
}

pub fn merge<T: Deserialize>(value: &mut T, bytes: &[u8]) {
    value.merge(&mut Reader::new(bytes)).unwrap();
}

pub fn assert_merge<T: PartialEq + fmt::Debug + Deserialize>(
    mut value: T,
    bytes: &[u8],
    expected_value: T,
) {
    merge(&mut value, bytes);
    assert_eq!(value, expected_value);
}

pub fn deserialize<T: Deserialize>(bytes: &[u8]) -> T {
    T::deserialize(&mut Reader::new(bytes)).unwrap()
}

pub fn assert_deserialize<T: PartialEq + fmt::Debug + Deserialize>(bytes: &[u8], value: T) {
    assert_eq!(deserialize::<T>(bytes), value);
}

pub fn assert_ser_de<T: Clone + PartialEq + fmt::Debug + Serialize + Deserialize>(value: T) {
    assert_eq!(deserialize::<T>(&*serialize(value.clone())), value);
}

pub fn replay<T: State>(value: &mut T, bytes: &[u8]) {
    value.replay(&mut Reader::new(bytes)).unwrap();
}
