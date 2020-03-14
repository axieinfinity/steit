extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod attr;
mod ctx;
mod derive;
mod r#impl;
mod string_util;

#[proc_macro_attribute]
pub fn steitize(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::do_it(args, input).into()
}
