#[derive(PartialEq, Eq, Hash, Debug)]
pub struct NameMeta {
    pub rust: &'static str,
    pub csharp: Option<&'static str>,
}

impl NameMeta {
    pub const fn new(rust_name: &'static str) -> Self {
        Self {
            rust: rust_name,
            csharp: None,
        }
    }

    pub fn csharp(&self, transform: impl FnOnce(&'static str) -> String) -> String {
        match self.csharp {
            Some(csharp) => transform(csharp),
            None => transform(self.rust),
        }
    }
}
