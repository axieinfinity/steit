#![allow(non_snake_case)]

use std::io::{self, Read};

use crate::{
    de_v2::{DeserializeV2, Reader},
    rt::SizeCache,
    ser_v2::SerializeV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name),+> HasWireType for ($($name),+) {
            const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
        }

        impl<$($name: SerializeV2),+> SerializeV2 for ($($name),+) {
            fn compute_size(&self) -> u32 {
                let ($($name),+) = self;
                let mut size = 0;
                $(size += $name.compute_size_nested_v2(None, false).unwrap();)+
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

        impl<$($name: DeserializeV2),+> DeserializeV2 for ($($name),+) {
            fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                $(let $name = $name::deserialize_nested_v2($name::WIRE_TYPE, reader)?;)+
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
