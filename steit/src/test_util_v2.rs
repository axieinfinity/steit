use std::fmt;

use super::{
    de_v2::{DeserializeV2, Reader},
    ser_v2::SerializeV2,
};

// #[macro_export]
// macro_rules! test_case {
//     ($name:ident : $assert:expr ; $($args:expr),+) => {
//         #[test]
//         fn $name() {
//             $assert($($args),+);
//         }
//     };
//
//     ($name:ident : $assert:expr ; $($input:expr),+ => $($output:expr),+) => {
//         test_case!($name : $assert ; $($input),+, $($output),+);
//     };
// }
//
// #[steitize(Serialize, Deserialize, own_crate)]
// #[derive(PartialEq, Debug)]
// pub struct Foo(#[steit(tag = 0)] i32, #[steit(tag = 1)] i32);
//
// impl Foo {
//     pub fn with(f0: i32, f1: i32) -> Self {
//         Self {
//             0: f0,
//             1: f1,
//             ..Foo::new()
//         }
//     }
// }
//
// #[steitize(State, own_crate)]
// #[derive(PartialEq, Debug)]
// pub struct Point {
//     #[steit(tag = 0)]
//     x: i32,
//     #[steit(tag = 1)]
//     y: i32,
//     #[steit(tag = 2)]
//     z: i32,
// }
//
// impl Point {
//     pub fn with(runtime: Runtime, x: i32, y: i32, z: i32) -> Self {
//         Self {
//             x,
//             y,
//             z,
//             ..Point::new(runtime)
//         }
//     }
//
//     pub fn x(&self) -> i32 {
//         self.x
//     }
// }

pub fn assert_size(value: impl SerializeV2, size: u32) {
    assert_eq!(value.compute_size(), size);
}

pub fn serialize(value: impl SerializeV2) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.serialize(&mut bytes).unwrap();
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

pub fn merge<T: DeserializeV2>(mut value: T, bytes: &[u8]) -> T {
    value.merge_v2(&mut Reader::new(bytes)).unwrap();
    value
}

pub fn assert_merge<T: PartialEq + fmt::Debug + DeserializeV2>(
    value: T,
    bytes: &[u8],
    expected_value: T,
) {
    assert_eq!(merge(value, bytes), expected_value);
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

// pub fn replay<T: State>(mut value: T, bytes: &[u8]) -> T {
//     value.replay(&mut Eof::new(bytes)).unwrap();
//     value
// }
