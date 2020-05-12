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
            fn handle_update_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                *self = <Self as $crate::de_v2::DeserializeV2>::deserialize_v2(reader)?;
                Ok(())
            }

            fn handle_v2(
                &mut self,
                path: impl Iterator<Item = u32>,
                kind: $crate::log::LogEntryKind,
                reader: &mut $crate::de_v2::Reader<impl ::std::io::Read>,
            ) -> io::Result<()> {
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
