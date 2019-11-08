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

use derive::DeriveKind;

#[proc_macro_derive(Serialize, attributes(steit))]
pub fn serialize_old(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::Serialize, input)
}

#[proc_macro_derive(Deserialize, attributes(steit))]
pub fn deserialize_old(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::Deserialize, input)
}

#[proc_macro_derive(State, attributes(steit))]
pub fn state_old(input: TokenStream) -> TokenStream {
    derive::derive(&DeriveKind::State, input)
}

#[proc_macro_attribute]
pub fn serialize(args: TokenStream, input: TokenStream) -> TokenStream {
    derive2::derive(derive2::DeriveKind::Serialize, args, input)
}

#[proc_macro_attribute]
pub fn deserialize(args: TokenStream, input: TokenStream) -> TokenStream {
    derive2::derive(derive2::DeriveKind::Deserialize, args, input)
}

#[proc_macro_attribute]
pub fn state(args: TokenStream, input: TokenStream) -> TokenStream {
    derive2::derive(derive2::DeriveKind::State, args, input)
}
