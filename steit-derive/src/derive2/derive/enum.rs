use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::derive2::{ctx::Context, derive, r#impl::Impl};

use super::{r#struct::Struct, variant::Variant, DeriveKind};

pub struct Enum<'a> {
    variants: Vec<Struct<'a>>,
}

impl<'a> Enum<'a> {
    pub fn parse(
        derive: &'a DeriveKind,
        context: &'a Context,
        r#impl: &'a Impl<'_>,
        data: &'a mut syn::DataEnum,
    ) -> derive::Result<Self> {
        if data.variants.is_empty() {
            context.error(&data.variants, "cannot derive for enums with zero variants");
            return Err(());
        }

        Self::parse_variants(derive, context, r#impl, &mut data.variants)
            .map(|variants| Self { variants })
    }

    fn parse_variants(
        derive: &'a DeriveKind,
        context: &'a Context,
        r#impl: &'a Impl<'_>,
        variants: &'a mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> derive::Result<Vec<Struct<'a>>> {
        let len = variants.iter().len();
        let mut parsed_data = Vec::with_capacity(len);

        for variant in variants.iter() {
            if variant.discriminant.is_some() {
                context.error(&variants, "cannot derive for enums with discriminants");
                return Err(());
            }
        }

        for variant in variants.iter_mut() {
            if let Ok((parsed, unknown_attrs)) = Variant::parse(context, variant) {
                parsed_data.push((parsed, unknown_attrs, &mut variant.fields));
            }
        }

        if parsed_data.len() != len {
            return Err(());
        }

        let mut tags = HashSet::new();
        let mut unique = true;

        for (variant, _, _) in &parsed_data {
            let (tag, tokens) = variant.tag_with_tokens();

            if !tags.insert(tag) {
                context.error(tokens, "duplicate tag");
                unique = false;
            }
        }

        let mut parsed = Vec::with_capacity(len);

        for (variant, unknown_attrs, fields) in parsed_data {
            if let Ok(r#struct) =
                Struct::parse(derive, context, r#impl, unknown_attrs, fields, variant)
            {
                parsed.push(r#struct);
            }
        }

        if parsed.len() != len {
            return Err(());
        }

        if unique {
            Ok(parsed)
        } else {
            Err(())
        }
    }
}

impl<'a> ToTokens for Enum<'a> {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
