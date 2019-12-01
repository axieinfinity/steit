use std::io;

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Deserialize, Eof, Merge, Serialize,
};

impl<T> WireType for Option<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Serialize> Serialize for Option<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        match self {
            Some(value) => value.compute_size_nested(None),
            None => 0,
        }
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Some(value) => value.serialize_nested_with_cached_size(None, writer),
            None => Ok(()),
        }
    }
}

impl<T: Deserialize> Merge for Option<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if !reader.eof()? {
            while !reader.eof()? {
                let value = T::deserialize_nested(reader)?;
                *self = Some(value);
            }
        } else {
            *self = None;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_size},
    };

    test_case!(size_01: assert_size; None::<u8> => 0);
    test_case!(size_02: assert_size; Some(0) => 1);
    test_case!(size_03: assert_size; Some(1) => 1);
    test_case!(size_04: assert_size; Some(1337) => 2);

    test_case!(serialize_01: assert_serialize; None::<u8> => &[]);
    test_case!(serialize_02: assert_serialize; Some(0) => &[0]);
    test_case!(serialize_03: assert_serialize; Some(1337) => &[242, 20]);

    test_case!(merge_01: assert_merge; Some(1), &[] => None);
    test_case!(merge_02: assert_merge; None, &[0] => Some(0));
    test_case!(merge_03: assert_merge; Some(0), &[242, 20] => Some(1337));
}
