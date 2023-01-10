#[macro_export]
macro_rules! impl_serialize_primitive {
    ($type:ty, $compute_size:ident, $serialize:ident) => {
        impl $crate::ser::Serialize for $type {
            fn compute_size(&self) -> u32 {
                $compute_size(self)
            }

            fn serialize_cached(
                &self,
                writer: &mut impl ::std::io::Write,
            ) -> ::std::io::Result<()> {
                self.steit_serialize(writer)
            }

            fn size_cache(&self) -> Option<&$crate::rt::SizeCache> {
                None
            }

            fn cache_size(&self) -> u32 {
                self.compute_size()
            }

            fn cached_size(&self) -> u32 {
                self.compute_size()
            }

            fn steit_serialize(&self, writer: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                $serialize(self, writer)
            }

            fn is_omissible(&self, _size_hint: Option<u32>) -> bool {
                *self == Self::default()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_state_primitive {
    ($type:ty) => {
        impl $crate::state::State for $type {
            fn with_runtime(_runtime: $crate::rt::Runtime) -> Self {
                Self::default()
            }

            fn runtime(&self) -> &$crate::rt::Runtime {
                panic!("cannot get `Runtime` from `{}`", stringify!($type))
            }

            fn set_runtime(&mut self, _runtime: $crate::rt::Runtime) {}

            fn handle_update(
                &mut self,
                reader: &mut $crate::de::Reader<impl ::std::io::Read>,
            ) -> ::std::io::Result<()> {
                *self = <Self as $crate::de::Deserialize>::deserialize(reader)?;
                Ok(())
            }

            fn handle(
                &mut self,
                path: impl Iterator<Item = u32>,
                kind: $crate::log::LogEntryKind,
                _key: Option<u32>,
                reader: &mut $crate::de::Reader<impl ::std::io::Read>,
            ) -> ::std::io::Result<()> {
                let path: Vec<_> = path.collect();

                if path.is_empty() {
                    match kind {
                        $crate::log::LogEntryKind::Update => self.handle_update(reader),

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
    ($type:ty, $csharp_name:literal, $wire_type:expr) => {
        impl $crate::meta::HasMeta for $type {
            const NAME: &'static $crate::meta::NameMeta = &$crate::meta::NameMeta {
                rust: stringify!($type),
                csharp: Some($csharp_name),
            };

            const TYPE: &'static $crate::meta::TypeMeta =
                &$crate::meta::TypeMeta::Primitive(Self::NAME, $wire_type);

            const LINK: &'static $crate::meta::MetaLink = &$crate::meta::MetaLink {
                r#type: Self::TYPE,
                msg: None,
                links: || &[],
            };
        }
    };
}
