extern crate proc_macro;
#[macro_use]
extern crate quote;

mod field;
mod r#struct;
mod util;

use r#struct::Struct;

#[proc_macro_derive(State, attributes(state))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let output = match input.data {
        syn::Data::Enum(ref data) => impl_enum(&input, data),
        syn::Data::Struct(ref data) => impl_struct(&input, &data.fields, None),
        syn::Data::Union(_) => unimplemented!("doesn't work with unions yet"),
    };

    output.into()
}

fn impl_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    if data.variants.is_empty() {
        panic!("cannot derive State for enums with zero variants");
    }

    let impls = data.variants.iter().map(|variant| {
        if variant.discriminant.is_some() {
            panic!("cannot derive State for enums with discriminants");
        }

        impl_struct(input, &variant.fields, Some(&variant.ident))
    });

    quote!(#(#impls)*)
}

fn impl_struct(
    input: &syn::DeriveInput,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let r#impl = |fields: Option<&syn::punctuated::Punctuated<_, _>>, named| {
        let fields = fields.expect("expected at least one field which is `vector_state::Path`");
        let r#struct = Struct::new(&input, fields, named, variant);
        quote!(#r#struct)
    };

    match *fields {
        syn::Fields::Named(ref fields) => r#impl(Some(&fields.named), true),
        syn::Fields::Unnamed(ref fields) => r#impl(Some(&fields.unnamed), false),
        syn::Fields::Unit => r#impl(None, false),
    }
}
