use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::derive2::{ctx::Context, derive, r#impl::Impl};

use super::{variant::Variant, DeriveKind};

pub struct Struct<'a> {
    derive: &'a DeriveKind,
    context: &'a Context,
    r#impl: &'a Impl,
    fields: Option<&'a mut syn::punctuated::Punctuated<syn::Field, syn::Token![,]>>,
    variant: Option<Variant>,
}

impl<'a> Struct<'a> {
    pub fn parse(
        derive: &'a DeriveKind,
        context: &'a Context,
        r#impl: &'a Impl,
        fields: &'a mut syn::Fields,
        variant: impl Into<Option<&'a mut syn::Variant>>,
    ) -> derive::Result<Self> {
        let fields = match fields {
            syn::Fields::Named(fields) => Some(&mut fields.named),
            syn::Fields::Unnamed(fields) => Some(&mut fields.unnamed),
            syn::Fields::Unit => None,
        };

        let variant = if let Some(variant) = variant.into() {
            match Variant::parse(context, variant) {
                Ok(variant) => Some(variant),
                Err(error) => return Err(error),
            }
        } else {
            None
        };

        Ok(Self {
            derive,
            context,
            r#impl,
            fields,
            variant,
        })
    }
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
