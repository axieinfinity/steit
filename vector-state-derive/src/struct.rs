use std::collections::HashSet;

use crate::context::Context;
use crate::field::{IndexedField, PathField};
use crate::util;

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
    path: PathField,
    indexed: Vec<IndexedField<'a>>,
}

impl<'a> Struct<'a> {
    pub fn parse<O: quote::ToTokens>(
        context: &Context,
        input: &'a syn::DeriveInput,
        object: O,
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        variant: Option<&syn::Ident>,
    ) -> Result<Self, ()> {
        let (paths, indexed): (Vec<_>, _) =
            fields.iter().enumerate().partition(|&(_index, field)| {
                match util::type_name(context, &field.ty) {
                    Some(ident) if ident == "Path" => true,
                    _ => false,
                }
            });

        if paths.len() == 0 {
            context.error(object, "expected exactly one `Path` field, got none");
            return Err(());
        }

        if paths.len() > 1 {
            context.error(fields, "expected exactly one `Path` field, got multiple");
            return Err(());
        }

        let path = paths
            .first()
            .map(|&(index, field)| PathField::new(field, index))
            .unwrap_or_else(|| unreachable!("expected a `Path` field"));

        Self::parse_indexed(context, indexed).map(|indexed| Self {
            input,
            path,
            indexed,
        })
    }

    fn parse_indexed(
        context: &Context,
        indexed: Vec<(usize, &'a syn::Field)>,
    ) -> Result<Vec<IndexedField<'a>>, ()> {
        let mut result = Vec::with_capacity(indexed.len());

        for (index, field) in &indexed {
            if let Ok(field) = IndexedField::parse(context, field, *index) {
                result.push(field);
            }
        }

        if result.len() != indexed.len() {
            return Err(());
        }

        let mut tags = HashSet::new();
        let mut unique = true;

        for field in &result {
            let tag = field.tag();

            if !tags.insert(*tag.value()) {
                context.error(tag, "duplicate tag");
                unique = false;
            }
        }

        if unique {
            Ok(result)
        } else {
            Err(())
        }
    }
}

impl<'a> quote::ToTokens for Struct<'a> {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {}
}
