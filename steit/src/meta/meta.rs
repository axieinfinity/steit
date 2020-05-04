use super::{message::MessageMeta, name::NameMeta, r#type::TypeMeta};

pub struct MetaLink {
    pub name: &'static NameMeta,
    pub message: Option<MessageMeta>,
    pub links: fn() -> &'static [&'static MetaLink],
}

pub trait HasMeta {
    const NAME: &'static NameMeta;
    const TYPE: &'static TypeMeta;
    const LINK: &'static MetaLink;
}
