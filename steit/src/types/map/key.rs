use std::io;

pub trait MapKey: Sized {
    fn try_from_field_number(field_number: u32) -> io::Result<Self>;
    fn as_field_number(&self) -> u32;
}

macro_rules! impl_map_key {
    ($type:ty) => {
        impl MapKey for $type {
            fn try_from_field_number(field_number: u32) -> io::Result<Self> {
                if field_number as u64 <= <$type>::MAX as u64 {
                    Ok(field_number as $type)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "field number {} is not within the range of `{}`",
                            field_number,
                            stringify!($type),
                        ),
                    ))
                }
            }

            fn as_field_number(&self) -> u32 {
                *self as u32
            }
        }
    };
}

impl_map_key!(u8);
impl_map_key!(u16);
impl_map_key!(u32);
