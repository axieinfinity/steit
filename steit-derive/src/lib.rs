#![feature(bind_by_move_pattern_guards)]

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod attr;
mod context;
mod derive;
mod derive2;
mod util;

use crate::{
    derive::DeriveKind,
    derive2::{derive2, Derive},
};

#[proc_macro_derive(Serialize, attributes(steit))]
pub fn serialize(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::Serialize, input)
}

#[proc_macro_derive(Deserialize, attributes(steit))]
pub fn deserialize(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::Deserialize, input)
}

#[proc_macro_derive(State, attributes(steit))]
pub fn state(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::State, input)
}

#[proc_macro_attribute]
pub fn serialize2(args: TokenStream, input: TokenStream) -> TokenStream {
    derive(Derive::Serialize, args, input)
}

fn derive(derive: Derive, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive2(derive, args, input).into()
}
