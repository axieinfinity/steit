use crate::{context::Context, r#struct::Struct, util};

#[derive(PartialEq)]
pub enum DeriveKind {
    Serialize,
    Deserialize,
    State,
}

pub fn derive(kind: &DeriveKind, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let context = Context::new();

    let output = match input.data {
        syn::Data::Enum(ref data) => impl_enum(&context, kind, &input, data),
        syn::Data::Struct(ref data) => impl_struct(
            &context,
            kind,
            &input,
            &data.struct_token,
            &data.fields,
            None,
        ),
        syn::Data::Union(ref data) => impl_union(&context, kind, &input, data),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(kind, &input.ident, output)
    };

    output.into()
}

fn impl_enum(
    context: &Context,
    kind: &DeriveKind,
    input: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    if data.variants.is_empty() {
        return context.error(&data.variants, "cannot derive for enums with zero variants");
    }

    let impls = data.variants.iter().map(|variant| {
        if variant.discriminant.is_some() {
            return context.error(&data.variants, "cannot derive for enums with discriminants");
        }

        impl_struct(
            context,
            kind,
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
    kind: &DeriveKind,
    input: &syn::DeriveInput,
    object: O,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let r#impl = |fields: &syn::punctuated::Punctuated<_, _>| {
        let r#struct = Struct::parse(&context, kind, &input, &object, fields, variant).ok();
        quote!(#r#struct)
    };

    match *fields {
        syn::Fields::Named(ref fields) => r#impl(&fields.named),
        syn::Fields::Unnamed(ref fields) => r#impl(&fields.unnamed),
        syn::Fields::Unit => context.error(object, "cannot derive for unit structs"),
    }
}

fn impl_union(
    context: &Context,
    _kind: &DeriveKind,
    _input: &syn::DeriveInput,
    data: &syn::DataUnion,
) -> proc_macro2::TokenStream {
    context.error(data.union_token, "cannot derive for unions yet")
}

fn wrap_in_const(
    kind: &DeriveKind,
    name: &syn::Ident,
    tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let r#const = format_ident!(
        "_IMPL_{}_FOR_{}",
        match kind {
            DeriveKind::Serialize => "SERIALIZE",
            DeriveKind::Deserialize => "DESERIALIZE",
            DeriveKind::State => "STATE",
        },
        util::to_snake_case(&name.to_string()).to_uppercase()
    );

    quote! {
        const #r#const: () = {
            extern crate steit;

            use std::io::{self, Read};

            use steit::{
                de::Deserialize,
                iowrap,
                ser::Serialize,
                // We don't import directly
                // to avoid confusing `serialize` and `deserialize` calls.
                varint,
            };

            #tokens
        };
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
