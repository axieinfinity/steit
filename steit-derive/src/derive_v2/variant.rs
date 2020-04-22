use proc_macro2::TokenStream;

use crate::{
    attr::{Attribute, AttributeParse},
    ctx::Context,
    str_util,
};

use super::{derive, tag};

struct VariantAttrs {
    tag: u32,
    tag_tokens: TokenStream,
    default: Option<(bool, TokenStream)>,
}

impl VariantAttrs {
    pub fn parse(
        ctx: &Context,
        variant: &mut syn::Variant,
    ) -> derive::Result<(Self, syn::AttributeArgs)> {
        let mut tag = Attribute::new(ctx, "tag");
        let mut default = Attribute::new(ctx, "default");

        let unknown_attrs = (&mut variant.attrs).parse(ctx, false, |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,

            syn::Meta::Path(path) if default.parse_path(path) => true,
            syn::Meta::NameValue(meta) if default.parse_bool(meta) => true,

            _ => false,
        });

        let (tag, tag_tokens) = tag
            .get_with_tokens()
            .ok_or_else(|| ctx.error(variant, "expected a valid tag #[steit(tag = …)]"))?;

        tag::validate(tag).map_err(|message| {
            ctx.error(&tag_tokens, message);
            ()
        })?;

        Ok((
            Self {
                tag,
                tag_tokens,
                default: default.get_with_tokens(),
            },
            unknown_attrs,
        ))
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
        let (attrs, unknown_attrs) = VariantAttrs::parse(ctx, variant)?;

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

    pub fn default_with_tokens(&self) -> (bool, Option<&TokenStream>) {
        match &self.attrs.default {
            Some((default, default_tokens)) => (*default, Some(default_tokens)),
            None => Default::default(),
        }
    }

    pub fn snake_case_name(&self) -> String {
        str_util::to_snake_case(self.name.to_string())
    }

    pub fn qual(&self) -> TokenStream {
        let name = &self.name;
        quote!(::#name)
    }

    pub fn ctor_name(&self) -> syn::Ident {
        format_ident!("empty_{}", self.snake_case_name())
    }
}
