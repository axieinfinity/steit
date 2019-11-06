use crate::derive2::{
    attr::{self, Attr},
    ctx::Context,
    derive,
};

struct VariantAttrs {
    tag: u16,
}

impl VariantAttrs {
    pub fn parse(context: &Context, attrs: &mut Vec<syn::Attribute>) -> derive::Result<Self> {
        let mut tag = Attr::new(context, "tag");

        attr::parse_attrs(context, attrs, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,
            _ => false,
        });

        if let Some(tag) = tag.get() {
            Ok(Self { tag })
        } else {
            Err(())
        }
    }
}

pub struct Variant {
    ident: syn::Ident,
    tag: u16,
}

impl Variant {
    pub fn parse(context: &Context, variant: &mut syn::Variant) -> derive::Result<Self> {
        VariantAttrs::parse(context, &mut variant.attrs).map(|attrs| Self {
            ident: variant.ident.clone(),
            tag: attrs.tag,
        })
    }
}
