use std::fmt;

use super::{steitize, Deserialize, Eof, Merge, Runtime, Serialize, State};

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

#[steitize(Serialize, Deserialize, own_crate)]
#[derive(PartialEq, Debug)]
pub struct Foo(#[steit(tag = 0)] i32, #[steit(tag = 1)] i32);

impl Foo {
    #[allow(dead_code)] // Since this function is mostly used in macros.
    pub fn with(f_0: i32, f_1: i32) -> Self {
        Self {
            0: f_0,
            1: f_1,
            ..Foo::new()
        }
    }
}

#[steitize(State, own_crate)]
#[derive(PartialEq, Debug)]
pub struct Point {
    #[steit(tag = 0)]
    x: i32,
    #[steit(tag = 1)]
    y: i32,
    #[steit(tag = 2)]
    z: i32,
}

impl Point {
    pub fn with(runtime: Runtime, x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            ..Point::new(runtime)
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_size<T: Serialize>(value: T, size: u32) {
    assert_eq!(value.compute_size(), size);
}

pub fn serialize<T: Serialize>(value: T) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.serialize(&mut bytes).unwrap();
    bytes
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_serialize<T: Serialize>(value: T, bytes: &[u8]) {
    assert_eq!(&*serialize(value), bytes);
}

pub fn serialize_nested<T: Serialize>(value: T, tag: Option<u16>) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.compute_size();
    value
        .serialize_nested_with_cached_size(tag, &mut bytes)
        .unwrap();
    bytes
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_serialize_nested<T: Serialize>(value: T, tag: Option<u16>, bytes: &[u8]) {
    assert_eq!(&*serialize_nested(value, tag), bytes);
}

pub fn merge<T: Merge>(mut value: T, bytes: &[u8]) -> T {
    value.merge(&mut Eof::new(bytes)).unwrap();
    value
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_merge<T: PartialEq + fmt::Debug + Merge>(value: T, bytes: &[u8], expected_value: T) {
    assert_eq!(merge(value, bytes), expected_value);
}

pub fn deserialize<T: Deserialize>(bytes: &[u8]) -> T {
    T::deserialize(&mut Eof::new(bytes)).unwrap()
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_deserialize<T: PartialEq + fmt::Debug + Deserialize>(bytes: &[u8], value: T) {
    assert_eq!(deserialize::<T>(bytes), value);
}

#[allow(dead_code)] // Since this function is mostly used in macros.
pub fn assert_ser_de<T: Clone + PartialEq + fmt::Debug + Serialize + Deserialize>(value: T) {
    assert_eq!(deserialize::<T>(&*serialize(value.clone())), value);
}

pub fn replay<T: State>(mut value: T, bytes: &[u8]) -> T {
    value.replay(&mut Eof::new(bytes)).unwrap();
    value
}
