extern crate proc_macro;
#[macro_use]
extern crate quote;

mod attr;
mod context;
mod internal;
mod util;

use crate::internal::DeriveKind;

#[proc_macro_derive(Serialize, attributes(steit))]
pub fn serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::derive(&DeriveKind::Serialize, input)
}

#[proc_macro_derive(Deserialize, attributes(steit))]
pub fn deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::derive(&DeriveKind::Deserialize, input)
}

#[proc_macro_derive(State, attributes(steit))]
pub fn state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::derive(&DeriveKind::State, input)
}
