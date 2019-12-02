use std::io::{self, Read};

use crate::{
    impl_state_for_plain,
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Eof, Merge, Serialize, State,
};

#[derive(PartialEq, Default, Debug)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    #[inline]
    pub fn with_raw(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[inline]
    pub fn with_value(value: &impl Serialize) -> Self {
        let mut bytes = Vec::new();
        value.serialize(&mut bytes).unwrap();
        Self { bytes }
    }

    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

impl WireType for Bytes {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Serialize for Bytes {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.bytes.len() as u32
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }
}

impl Merge for Bytes {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.bytes)?;
        Ok(())
    }
}

impl State for Bytes {
    impl_state_for_plain!("bytes");
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_size, Foo},
    };

    use super::Bytes;

    test_case!(size_01: assert_size; Bytes::with_value(&None::<u8>) => 0);
    test_case!(size_02: assert_size; Bytes::with_value(&Some(1337)) => 2);
    test_case!(size_03: assert_size; Bytes::with_value(&0) => 1);
    test_case!(size_04: assert_size; Bytes::with_value(&Foo::new()) => 0);
    test_case!(size_05: assert_size; Bytes::with_value(&Foo::with(-1, -1)) => 4);

    test_case!(serialize_01: assert_serialize; Bytes::with_value(&None::<u8>) => &[]);
    test_case!(serialize_02: assert_serialize; Bytes::with_value(&Some(1337)) => &[242, 20]);
    test_case!(serialize_03: assert_serialize; Bytes::with_value(&0) => &[0]);
    test_case!(serialize_04: assert_serialize; Bytes::with_value(&Foo::new()) => &[]);
    test_case!(serialize_05: assert_serialize; Bytes::with_value(&Foo::with(-1, -1)) => &[0, 1, 8, 1]);

    test_case!(merge_01: assert_merge; Bytes::with_value(&None::<u8>), &[242, 20] => Bytes::with_value(&Some(1337)));
    test_case!(merge_02: assert_merge; Bytes::with_value(&Foo::with(-1, -1)), &[8, 3] => Bytes::with_raw(vec![0, 1, 8, 1, 8, 3]));
}
