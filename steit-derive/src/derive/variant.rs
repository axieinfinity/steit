use proc_macro2::TokenStream;

use crate::{
    attr::{Attr, AttrParse},
    ctx::Context,
    derive, string,
};

struct VariantAttrs {
    tag: u16,
    tag_tokens: TokenStream,
}

impl VariantAttrs {
    pub fn parse(
        context: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        let mut tag = Attr::new(context, "tag");

        let unknown_attrs = (&mut variant.attrs).parse(context, false, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,
            _ => false,
        });

        if let Some((tag, tag_tokens)) = tag.get_with_tokens() {
            Ok((Self { tag, tag_tokens }, unknown_attrs))
        } else {
            context.error(variant, "expected a valid tag #[steit(tag = ...)]");
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
        context: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        VariantAttrs::parse(context, variant).map(|(attrs, unknown_attrs)| {
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

    pub fn qual(&self) -> TokenStream {
        let name = &self.name;
        quote!(::#name)
    }

    pub fn ctor_name(&self) -> syn::Ident {
        let name = string::to_snake_case(&self.name.to_string());
        format_ident!("new_{}", name)
    }
}
