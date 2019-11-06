use proc_macro2::TokenStream;
use quote::ToTokens;

use super::ImplInput;

pub struct Enum;

impl ToTokens for Enum {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unimplemented!()
    }
}

pub fn r#impl(_input: ImplInput, _data: syn::DataEnum) -> Enum {
    Enum
}
