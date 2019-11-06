use proc_macro2::TokenStream;
use quote::ToTokens;

use super::ImplInput;

pub struct Union;

impl ToTokens for Union {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unimplemented!()
    }
}

pub fn r#impl(_input: ImplInput, _data: syn::DataUnion) -> Union {
    Union
}
