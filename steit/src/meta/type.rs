use super::message::MessageMeta;

#[derive(Debug)]
pub enum TypeMeta {
    Primitive(&'static str),
    Message(&'static MessageMeta),
    MessageRef(&'static str),
    Vec(&'static TypeMeta),
    List(&'static TypeMeta),
    Map(&'static TypeMeta),
}

pub trait HasTypeMeta {
    const TYPE_META: &'static TypeMeta;
    const TYPE_REF_META: &'static TypeMeta = Self::TYPE_META;
}
