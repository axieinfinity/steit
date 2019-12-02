use std::io::{self, Read};

use crate::{
    types::Varint,
    wire_type::{WireType, WIRE_TYPE_VARINT},
    Eof, Merge, Serialize,
};

impl WireType for bool {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;
}

impl Serialize for bool {
    #[inline]
    fn compute_size(&self) -> u32 {
        1
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.serialize_with_cached_size(writer)
    }

    #[inline]
    fn is_default_nested_with_cached_size(&self) -> bool {
        !*self
    }
}

impl Merge for bool {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        let mut value = false;
        let mut buf = [0];

        loop {
            reader.read_exact(&mut buf)?;
            value |= buf[0] & 0x7f != 0;

            if buf[0] & 0x80 == 0 {
                *self = value;
                return Ok(());
            }
        }
    }
}

impl Varint for bool {}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize},
    };

    test_case!(serialize_01: assert_serialize; false => &[0]);
    test_case!(serialize_02: assert_serialize; true => &[1]);

    test_case!(merge_01: assert_merge; false, &[0] => false);
    test_case!(merge_02: assert_merge; false, &[1] => true);
    test_case!(merge_03: assert_merge; false, &[128, 128, 128, 128, 128, 128, 128, 128, 128, 2] /* 2^64 */ => true);
    test_case!(merge_04: assert_merge; true, &[0] => false);
    test_case!(merge_05: assert_merge; true, &[42] => true);
}
