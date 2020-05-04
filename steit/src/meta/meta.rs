use super::{msg::MessageMeta, name::NameMeta, r#type::TypeMeta};

pub struct MetaLink {
    pub r#type: &'static TypeMeta,
    pub msg: Option<MessageMeta>,
    pub links: fn() -> &'static [&'static MetaLink],
}

pub trait HasMeta {
    const NAME: &'static NameMeta;
    const TYPE: &'static TypeMeta;
    const LINK: &'static MetaLink;
}
