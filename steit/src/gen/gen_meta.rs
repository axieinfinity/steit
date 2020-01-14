#[derive(Debug)]
pub enum Meta {
    Struct(&'static Struct),
    Enum(&'static Enum),
}

#[derive(Debug)]
pub struct Struct {
    pub name: &'static str,
    pub fields: &'static [Field],
}

#[derive(Debug)]
pub struct Enum {
    pub name: &'static str,
    pub variants: &'static [Variant],
}

#[derive(Debug)]
pub struct Variant {
    pub ty: &'static Struct,
    pub tag: u16,
}

impl Variant {
    pub fn is_default(&self) -> bool {
        self.tag == 0
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static FieldType,
    pub tag: u16,
}

#[derive(Debug)]
pub enum FieldType {
    Primitive(&'static str),
    Meta(&'static Meta),
    MetaRef(&'static str),
    List(&'static FieldType),
    Map(&'static FieldType),
}

pub trait HasMeta {
    const META: &'static Meta;
    const META_NAME: &'static str;
}

pub trait IsFieldType {
    const FIELD_TYPE: &'static FieldType;
    const FIELD_TYPE_REF: &'static FieldType = Self::FIELD_TYPE;
}

macro_rules! impl_has_field_type {
    ($t:ty, $t_name:expr) => {
        impl IsFieldType for $t {
            const FIELD_TYPE: &'static FieldType = &FieldType::Primitive($t_name);
        }
    };
}

impl_has_field_type!(u8, "u8");
impl_has_field_type!(u16, "u16");
impl_has_field_type!(u32, "u32");
impl_has_field_type!(u64, "u64");
impl_has_field_type!(i8, "i8");
impl_has_field_type!(i16, "i16");
impl_has_field_type!(i32, "i32");
impl_has_field_type!(i64, "i64");
impl_has_field_type!(bool, "bool");
