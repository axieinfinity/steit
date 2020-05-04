use super::{name::NameMeta, r#type::FieldTypeMeta};

#[derive(PartialEq, Debug)]
pub enum MessageMeta {
    Struct(StructMeta),
    Enum(EnumMeta),
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

#[derive(PartialEq, Debug)]
pub struct StructMeta {
    pub name: &'static NameMeta,
    pub fields: &'static [FieldMeta],
    pub builtin: bool,
}

#[derive(PartialEq, Debug)]
pub struct EnumMeta {
    pub name: &'static NameMeta,
    pub variants: &'static [VariantMeta],
    pub builtin: bool,
}

#[derive(PartialEq, Debug)]
pub struct VariantMeta {
    pub ty: StructMeta,
    pub tag: u32,
    pub default: bool,
}

#[derive(PartialEq, Debug)]
pub struct FieldMeta {
    pub name: &'static NameMeta,
    pub ty: &'static FieldTypeMeta,
    pub tag: u32,
}
