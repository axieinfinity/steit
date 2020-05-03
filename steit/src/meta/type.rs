use super::{message::MessageMeta, name::NameMeta};

#[derive(Debug)]
pub enum TypeMeta {
    Primitive(&'static NameMeta),
    Message(&'static MessageMeta),
    MessageRef(&'static NameMeta),
    Vec(&'static TypeMeta),
    List(&'static TypeMeta),
    Map(&'static TypeMeta),
}

pub trait HasTypeMeta {
    const TYPE_NAME: &'static NameMeta;
    const TYPE_META: &'static TypeMeta;
    const TYPE_REF_META: &'static TypeMeta = Self::TYPE_META;
}
