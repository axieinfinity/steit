#[derive(PartialEq, Debug)]
pub struct NameMeta {
    pub rust: &'static str,
    pub csharp: Option<&'static str>,
}

impl NameMeta {
    #[inline]
    pub const fn new(rust_name: &'static str) -> Self {
        Self {
            rust: rust_name,
            csharp: None,
        }
    }
}
