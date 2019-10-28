use proc_macro2::TokenStream;

use crate::field::IndexedField;
use crate::util;

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
}

impl<'a> Struct<'a> {
    pub fn new(
        input: &'a syn::DeriveInput,
        fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        named: bool,
        variant: Option<&syn::Ident>,
    ) -> Self {
        let (path, indexed) = Self::extract_path(fields);

        let indexed: Vec<_> = indexed
            .iter()
            .map(|&(index, field)| IndexedField::new(field, index))
            .collect();

        for field in indexed {
            field.debug();
        }

        Self { input }
    }

    fn extract_path(
        fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    ) -> ((usize, &syn::Field), Vec<(usize, &syn::Field)>) {
        let (paths, indexed): (Vec<_>, _) = fields
            .iter()
            .enumerate()
            .partition(|&(index, field)| util::type_name(&field.ty) == "Path");

        assert_eq!(
            paths.len(),
            1,
            "expected exactly one `vector_state::Path` field"
        );

        let path = *paths.first().unwrap_or_else(|| unreachable!());

        (path, indexed)
    }
}

impl<'a> quote::ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        unimplemented!()
    }
}
