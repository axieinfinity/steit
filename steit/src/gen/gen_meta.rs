#[derive(Debug)]
pub enum Meta {
    Struct(Struct),
    Enum(Enum),
    List(Field),
    Map(Field),
}

#[derive(Debug)]
pub enum State {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: &'static str,
    pub fields: &'static [Field],
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: &'static str,
    pub variants: &'static [Variant],
}

#[derive(Debug)]
pub struct Variant {
    pub ty: &'static Struct,
    pub tag: u16,
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
