use super::{name::NameMeta, r#type::TypeMeta};

#[derive(Debug)]
pub enum MessageMeta {
    Struct(&'static StructMeta),
    Enum(&'static EnumMeta),
}

impl MessageMeta {
    #[inline]
    pub fn is_builtin(&self) -> bool {
        match self {
            MessageMeta::Struct(StructMeta { builtin, .. })
            | MessageMeta::Enum(EnumMeta { builtin, .. }) => *builtin,
        }
    }
}

#[derive(Debug)]
pub struct StructMeta {
    pub name: &'static NameMeta,
    pub fields: &'static [FieldMeta],
    pub builtin: bool,
}

#[derive(Debug)]
pub struct EnumMeta {
    pub name: &'static NameMeta,
    pub variants: &'static [VariantMeta],
    pub builtin: bool,
}

#[derive(Debug)]
pub struct VariantMeta {
    pub ty: &'static StructMeta,
    pub tag: u32,
    pub default: bool,
}

#[derive(Debug)]
pub struct FieldMeta {
    pub name: &'static NameMeta,
    pub ty: &'static TypeMeta,
    pub tag: u32,
}

pub trait HasMessageMeta {
    const MESSAGE_NAME: &'static NameMeta;
    const MESSAGE_META: &'static MessageMeta;
}
