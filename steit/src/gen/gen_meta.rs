#[derive(Debug)]
pub enum Meta {
    State(&'static str, &'static [Field]),
    List(Field),
    Map(Field),
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
