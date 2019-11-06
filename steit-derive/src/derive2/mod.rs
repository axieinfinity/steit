use proc_macro::TokenStream;

mod attr;
mod ctx;
mod derive;
mod r#impl;
mod string;

pub use derive::DeriveKind;

pub fn derive(derive: DeriveKind, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::derive(derive, args, input).into()
}
