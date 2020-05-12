#[macro_export]
macro_rules! impl_serialize_primitive {
    ($type:ty, $compute_size:ident, $serialize:ident) => {
        impl $crate::ser_v2::SerializeV2 for $type {
            #[inline]
            fn compute_size_v2(&self) -> u32 {
                $compute_size(self)
            }

            #[inline]
            fn serialize_cached(
                &self,
                writer: &mut impl ::std::io::Write,
            ) -> ::std::io::Result<()> {
                self.serialize_v2(writer)
            }

            #[inline]
            fn size_cache(&self) -> Option<&$crate::rt::SizeCache> {
                None
            }

            #[inline]
            fn cache_size(&self) -> u32 {
                self.compute_size_v2()
            }

            #[inline]
            fn cached_size(&self) -> u32 {
                self.compute_size_v2()
            }

            #[inline]
            fn serialize_v2(&self, writer: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                $serialize(self, writer)
            }

            #[inline]
            fn is_omissible(&self) -> bool {
                *self == Self::default()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_state_primitive {
    ($type:ty) => {
        impl $crate::state_v2::StateV2 for $type {
            #[inline]
            fn with_runtime_v2(_runtime: $crate::rt::RuntimeV2) -> Self {
                Self::default()
            }

            #[inline]
            fn runtime_v2(&self) -> &$crate::rt::RuntimeV2 {
                panic!("cannot get `Runtime` from `{}`", stringify!($type))
            }

            #[inline]
            fn set_runtime_v2(&mut self, _runtime: $crate::rt::RuntimeV2) {}

            #[inline]
            fn handle_update_v2(
                &mut self,
                reader: &mut $crate::de_v2::Reader<impl ::std::io::Read>,
            ) -> ::std::io::Result<()> {
                *self = <Self as $crate::de_v2::DeserializeV2>::deserialize_v2(reader)?;
                Ok(())
            }

            fn handle_v2(
                &mut self,
                path: impl Iterator<Item = u32>,
                kind: $crate::log::LogEntryKind,
                reader: &mut $crate::de_v2::Reader<impl ::std::io::Read>,
            ) -> ::std::io::Result<()> {
                let path: Vec<_> = path.collect();

                if path.is_empty() {
                    match kind {
                        $crate::log::LogEntryKind::Update => self.handle_update_v2(reader),

                        _ => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!("{:?} is not supported on `{}`", kind, stringify!($name)),
                        )),
                    }
                } else {
                    Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "`{}` expected end-of-path but still got {:?} remaining",
                            stringify!($name),
                            path,
                        ),
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_meta_primitive {
    ($type:ty, $csharp_name:literal) => {
        impl $crate::meta::HasMeta for $type {
            const NAME: &'static $crate::meta::NameMeta = &$crate::meta::NameMeta {
                rust: stringify!($type),
                csharp: Some($csharp_name),
            };

            const TYPE: &'static $crate::meta::TypeMeta =
                &$crate::meta::TypeMeta::Primitive(Self::NAME);

            const LINK: &'static $crate::meta::MetaLink = &$crate::meta::MetaLink {
                r#type: Self::TYPE,
                msg: None,
                links: || &[],
            };
        }
    };
}
