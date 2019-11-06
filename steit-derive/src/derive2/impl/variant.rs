use proc_macro2::TokenStream;
use quote::ToTokens;

use super::ImplInput;

pub struct Variant;

impl ToTokens for Variant {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unimplemented!()
    }
}

pub fn r#impl(_input: ImplInput, _fields: syn::Fields) -> Variant {
    Variant
}
