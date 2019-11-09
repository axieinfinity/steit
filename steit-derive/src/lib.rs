#![feature(bind_by_move_pattern_guards)]

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod attr;
mod ctx;
mod derive;
mod r#impl;
mod string;

use derive::DeriveKind;

#[proc_macro_attribute]
pub fn serialize(args: TokenStream, input: TokenStream) -> TokenStream {
    derive(DeriveKind::Serialize, args, input)
}

#[proc_macro_attribute]
pub fn deserialize(args: TokenStream, input: TokenStream) -> TokenStream {
    derive(DeriveKind::Deserialize, args, input)
}

#[proc_macro_attribute]
pub fn state(args: TokenStream, input: TokenStream) -> TokenStream {
    derive(DeriveKind::State, args, input)
}

fn derive(derive: DeriveKind, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::derive(derive, args, input).into()
}
