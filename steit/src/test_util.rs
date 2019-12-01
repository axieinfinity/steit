use std::fmt;

use super::{Deserialize, Eof, Merge, Serialize};

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
