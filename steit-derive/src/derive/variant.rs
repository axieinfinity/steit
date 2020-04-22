use proc_macro2::TokenStream;

use crate::{
    attr::{Attribute, AttributeParse},
    ctx::Context,
    str_util,
};

use super::derive;

struct VariantAttrs {
    tag: u16,
    tag_tokens: TokenStream,
}

impl VariantAttrs {
    pub fn parse(
        ctx: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        let mut tag = Attribute::new(ctx, "tag");

        let unknown_attrs = (&mut variant.attrs).parse(ctx, false, |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,
            _ => false,
        });

        if let Some((tag, tag_tokens)) = tag.get_with_tokens() {
            Ok((Self { tag, tag_tokens }, unknown_attrs))
        } else {
            ctx.error(variant, "expected a valid tag #[steit(tag = ...)]");
            Err(())
        }
    }
}

pub struct Variant {
    name: syn::Ident,
    attrs: VariantAttrs,
}

impl Variant {
    pub fn parse(
        ctx: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        VariantAttrs::parse(ctx, variant).map(|(attrs, unknown_attrs)| {
            (
                Self {
                    name: variant.ident.clone(),
                    attrs,
                },
                unknown_attrs,
            )
        })
    }

    pub fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub fn tag(&self) -> u16 {
        self.attrs.tag
    }

    pub fn tag_with_tokens(&self) -> (u16, &TokenStream) {
        (self.attrs.tag, &self.attrs.tag_tokens)
    }

    pub fn snake_case_name(&self) -> String {
        str_util::to_snake_case(self.name.to_string())
    }

    pub fn qual(&self) -> TokenStream {
        let name = &self.name;
        quote!(::#name)
    }

    pub fn ctor_name(&self) -> syn::Ident {
        format_ident!("new_{}", self.snake_case_name())
    }
}
