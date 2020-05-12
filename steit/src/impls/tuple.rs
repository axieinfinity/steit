#![allow(non_snake_case)]

use std::io::{self, Read};

use crate::{
    de::{Deserialize, Reader},
    rt::SizeCache,
    ser::Serialize,
    wire_fmt::{HasWireType, WireType},
};

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name),+> HasWireType for ($($name),+) {
            const WIRE_TYPE: WireType = WireType::Sized;
        }

        impl<$($name: Serialize),+> Serialize for ($($name),+) {
            fn compute_size(&self) -> u32 {
                let ($($name),+) = self;
                let mut size = 0;
                $(size += $name.compute_size_nested(None, false).unwrap();)+
                size
            }

            fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
                let ($($name),+) = self;
                $($name.serialize_nested(None, false, writer)?;)+
                Ok(())
            }

            #[inline]
            fn size_cache(&self) -> Option<&SizeCache> {
                None
            }
        }

        impl<$($name: Deserialize),+> Deserialize for ($($name),+) {
            fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                $(let $name = $name::deserialize_nested($name::WIRE_TYPE, reader)?;)+
                *self = ($($name),+);

                let mut remaining = Vec::new();
                reader.read_to_end(&mut remaining)?;

                Ok(())
            }
        }
    };
}

tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
