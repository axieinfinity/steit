extern crate proc_macro;
#[macro_use]
extern crate quote;

mod attr;
mod context;
mod field;
mod r#struct;
mod util;

use crate::context::Context;
use crate::r#struct::Struct;

#[proc_macro_derive(State, attributes(state))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let context = Context::new();

    let output = match input.data {
        syn::Data::Enum(ref data) => impl_enum(&context, &input, data),
        syn::Data::Struct(ref data) => {
            impl_struct(&context, &input, &data.struct_token, &data.fields, None)
        }
        syn::Data::Union(ref data) => impl_union(&context, &input, data),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        output
    };

    output.into()
}

fn impl_enum(
    context: &Context,
    input: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    if data.variants.is_empty() {
        return context.error(
            &data.variants,
            "cannot #[derive(State)] for enums with zero variants",
        );
    }

    let impls = data.variants.iter().map(|variant| {
        if variant.discriminant.is_some() {
            return context.error(
                &data.variants,
                "cannot #[derive(State)] for enums with discriminants",
            );
        }

        impl_struct(
            context,
            input,
            &variant.ident,
            &variant.fields,
            Some(&variant.ident),
        )
    });

    quote!(#(#impls)*)
}

fn impl_struct<O: quote::ToTokens>(
    context: &Context,
    input: &syn::DeriveInput,
    object: O,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let r#impl = |fields: &syn::punctuated::Punctuated<_, _>| {
        let r#struct = Struct::parse(&context, &input, &object, fields, variant).ok();
        quote!(#r#struct)
    };

    match *fields {
        syn::Fields::Named(ref fields) => r#impl(&fields.named),
        syn::Fields::Unnamed(ref fields) => r#impl(&fields.unnamed),
        syn::Fields::Unit => context.error(object, "cannot #[derive(State)] for unit structs"),
    }
}

fn impl_union(
    context: &Context,
    _input: &syn::DeriveInput,
    data: &syn::DataUnion,
) -> proc_macro2::TokenStream {
    context.error(data.union_token, "cannot #[derive(State)] for unions yet")
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
