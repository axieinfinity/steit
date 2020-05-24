use std::{ffi::OsStr, ops::Deref, path::PathBuf};

pub struct Setting<T> {
    pub out_dir: PathBuf,
    pub skip_builtins: bool,
    inner: T,
}

impl<T> Deref for Setting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Setting<T> {
    pub fn new(out_dir: &impl AsRef<OsStr>, skip_builtins: bool, inner: T) -> Self {
        Self {
            out_dir: PathBuf::from(out_dir),
            skip_builtins,
            inner,
        }
    }
}
