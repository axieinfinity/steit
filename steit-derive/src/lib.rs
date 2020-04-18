extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod attr;
mod context;
mod derive;
mod derive_v2;
mod impl_util;
mod string_util;

#[proc_macro_attribute]
pub fn steitize(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::do_it(args, input).into()
}

#[proc_macro_attribute]
pub fn steit_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive_v2::do_it(args, input).into()
}
