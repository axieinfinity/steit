extern crate proc_macro;
#[macro_use]
extern crate quote;

mod attr;
mod context;
mod derivation;
mod field;
mod r#struct;

use crate::derivation::DerivationKind;

#[proc_macro_derive(Deserialize, attributes(steit))]
pub fn serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derivation::derive(&DerivationKind::Serialize, input)
}

#[proc_macro_derive(Serialize, attributes(steit))]
pub fn deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derivation::derive(&DerivationKind::Deserialize, input)
}

#[proc_macro_derive(State, attributes(steit))]
pub fn state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derivation::derive(&DerivationKind::State, input)
}
