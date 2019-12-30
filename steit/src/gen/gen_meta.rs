#[derive(Debug)]
pub enum Meta {
    Struct(&'static Struct),
    Enum(&'static Enum),
    List(FieldType),
    Map(FieldType),
}

#[derive(Debug)]
pub enum State {
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
}

pub trait HasMeta {
    const META: &'static Meta;
}
