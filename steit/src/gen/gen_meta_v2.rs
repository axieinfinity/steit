#[derive(Debug)]
pub enum MetaV2 {
    Struct(&'static StructV2),
    Enum(&'static EnumV2),
}

impl MetaV2 {
    #[inline]
    pub fn is_builtin(&self) -> bool {
        match self {
            MetaV2::Struct(StructV2 { builtin, .. }) | MetaV2::Enum(EnumV2 { builtin, .. }) => {
                *builtin
            }
        }
    }
}

#[derive(Debug)]
pub struct StructV2 {
    pub name: &'static str,
    pub fields: &'static [FieldV2],
    pub builtin: bool,
}

#[derive(Debug)]
pub struct EnumV2 {
    pub name: &'static str,
    pub variants: &'static [VariantV2],
    pub builtin: bool,
}

#[derive(Debug)]
pub struct VariantV2 {
    pub ty: &'static StructV2,
    pub tag: u32,
    pub default: bool,
}

#[derive(Debug)]
pub struct FieldV2 {
    pub name: &'static str,
    pub ty: &'static FieldTypeV2,
    pub tag: u32,
    pub csharp_name: Option<&'static str>,
}

#[derive(Debug)]
pub enum FieldTypeV2 {
    Primitive(&'static str),
    Meta(&'static MetaV2),
    MetaRef(&'static str),
    Vec(&'static FieldTypeV2),
    List(&'static FieldTypeV2),
    Map(&'static FieldTypeV2),
}

pub trait HasMetaV2 {
    const META: &'static MetaV2;
    const META_NAME: &'static str;
}

pub trait IsFieldTypeV2 {
    const FIELD_TYPE: &'static FieldTypeV2;
    const FIELD_TYPE_REF: &'static FieldTypeV2 = Self::FIELD_TYPE;
}

#[macro_export]
macro_rules! impl_field_type_primitive {
    ($type:ty) => {
        impl $crate::gen::IsFieldTypeV2 for $type {
            const FIELD_TYPE: &'static $crate::gen::FieldTypeV2 =
                &$crate::gen::FieldTypeV2::Primitive(stringify!($primitive));
        }
    };
}
