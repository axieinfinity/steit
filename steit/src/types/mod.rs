mod bytes;
mod impls;
mod stateful;
mod varint;

pub use bytes::*;
pub use impls::*;
pub use stateful::*;
pub use varint::*;

#[macro_export]
macro_rules! impl_state_for_plain {
    ($name:literal) => {
        fn with_runtime(_runtime: $crate::Runtime) -> Self {
            <Self as Default>::default()
        }

        #[inline]
        fn runtime(&self) -> &$crate::Runtime {
            panic!("cannot get `Runtime` from {}", $name)
        }

        #[inline]
        fn handle<'a>(
            &mut self,
            path: &mut impl Iterator<Item = &'a u16>,
            kind: &$crate::ReplayKind,
            reader: &mut $crate::Eof<impl ::std::io::Read>,
        ) -> ::std::io::Result<()> {
            if let Some(tag) = path.next() {
                let mut s = format!("{}", tag);

                for tag in path {
                    s.push_str(&format!(", {}", tag));
                }

                Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    format!("{} expected end-of-path but still got [{}] remaining", $name, s),
                ))
            } else {
                match kind {
                    $crate::ReplayKind::Update => self.handle_update(reader),

                    $crate::ReplayKind::Add | $crate::ReplayKind::Remove => Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("`add` and `remove` are not supported on {}", $name),
                    )),
                }
            }
        }

        #[inline]
        fn handle_update(
            &mut self,
            reader: &mut $crate::Eof<impl ::std::io::Read>,
        ) -> ::std::io::Result<()> {
            *self = <Self as $crate::Deserialize>::deserialize(reader)?;
            Ok(())
        }
    };
}
