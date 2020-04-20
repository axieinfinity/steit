use proc_macro2::TokenStream;

use crate::{
    attr::{Attr, AttrParse},
    context::Context,
};

use super::{derive, tag};

struct VariantAttrs {
    tag: u32,
    tag_tokens: TokenStream,
}

impl VariantAttrs {
    pub fn parse(
        context: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        let mut tag = Attr::new(context, "tag");

        let unknown_attrs = (&mut variant.attrs).parse(context, false, |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,
            _ => false,
        });

        let (tag, tag_tokens) = tag
            .get_with_tokens()
            .ok_or_else(|| context.error(variant, "expected a valid tag #[steit(tag = …)]"))?;

        tag::validate(tag).map_err(|message| {
            context.error(&tag_tokens, message);
            ()
        })?;

        Ok((Self { tag, tag_tokens }, unknown_attrs))
    }
}

pub struct Variant {
    name: syn::Ident,
    attrs: VariantAttrs,
}

impl Variant {
    pub fn parse(
        context: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        let (attrs, unknown_attrs) = VariantAttrs::parse(context, variant)?;

        Ok((
            Self {
                name: variant.ident.clone(),
                attrs,
            },
            unknown_attrs,
        ))
    }

    pub fn tag(&self) -> u32 {
        self.attrs.tag
    }

    pub fn tag_with_tokens(&self) -> (u32, &TokenStream) {
        (self.attrs.tag, &self.attrs.tag_tokens)
    }

    pub fn qual(&self) -> TokenStream {
        let name = &self.name;
        quote!(::#name)
    }
}
