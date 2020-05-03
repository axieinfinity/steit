use super::r#type::TypeMeta;

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
    pub name: &'static str,
    pub fields: &'static [FieldMeta],
    pub builtin: bool,
}

#[derive(Debug)]
pub struct EnumMeta {
    pub name: &'static str,
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
    pub name: &'static str,
    pub ty: &'static TypeMeta,
    pub tag: u32,
    pub csharp_name: Option<&'static str>,
}

pub trait HasMessageMeta {
    const MESSAGE_NAME: &'static str;
    const MESSAGE_META: &'static MessageMeta;
}
