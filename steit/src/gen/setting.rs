use std::{collections::HashSet, ffi::OsStr, ops::Deref, path::PathBuf};

use crate::meta::MessageMeta;

pub struct Setting<T> {
    pub(in crate::gen) out_dir: PathBuf,
    pub(in crate::gen) get_name: fn(&'static MessageMeta) -> String,
    pub(in crate::gen) skip_builtins: bool,
    pub(in crate::gen) skip_names: HashSet<String>,
    inner: T,
}

impl<T> Deref for Setting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

const GET_NAME_RUST: fn(&MessageMeta) -> String = |msg| msg.rust_name().to_string();
const GET_NAME_CSHARP: fn(&MessageMeta) -> String = MessageMeta::csharp_name;

impl<T> Setting<T> {
    pub fn new(out_dir: &impl AsRef<OsStr>, inner: T) -> Self {
        Self {
            out_dir: PathBuf::from(out_dir),
            get_name: GET_NAME_CSHARP,
            skip_builtins: true,
            skip_names: HashSet::new(),
            inner,
        }
    }

    pub fn get_name_rust(mut self) -> Self {
        self.get_name = GET_NAME_RUST;
        self
    }

    pub fn get_name_csharp(mut self) -> Self {
        self.get_name = GET_NAME_CSHARP;
        self
    }

    pub fn skip_builtins(mut self, skip_builtins: bool) -> Self {
        self.skip_builtins = skip_builtins;
        self
    }

    pub fn skip_names(mut self, skip_names: Vec<String>) -> Self {
        self.skip_names = skip_names.into_iter().collect();
        self
    }
}
