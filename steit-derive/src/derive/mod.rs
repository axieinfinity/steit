use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    ctx::Context,
    r#impl::Impl,
    string,
};

mod r#enum;
mod field;
mod r#struct;
mod union;
mod variant;

use r#enum::Enum;
use r#struct::Struct;
use union::Union;

pub type Result<T> = std::result::Result<T, ()>;

#[derive(PartialEq)]
pub enum DeriveKind {
    Serialize,
    Deserialize,
    State,
}

struct DeriveAttrs {
    own_crate: bool,
}

impl DeriveAttrs {
    pub fn parse(context: &Context, args: syn::AttributeArgs) -> Self {
        let mut own_crate = Attr::new(context, "own_crate");

        args.parse(context, true, &mut |meta| match meta {
            syn::Meta::Path(path) if own_crate.parse_path(path) => true,
            syn::Meta::NameValue(meta) if own_crate.parse_bool(meta) => true,
            _ => false,
        });

        Self {
            own_crate: own_crate.get().unwrap_or_default(),
        }
    }
}

pub fn derive(
    derive: DeriveKind,
    args: syn::AttributeArgs,
    mut input: syn::DeriveInput,
) -> TokenStream {
    let context = Context::new();
    let attrs = DeriveAttrs::parse(&context, args);
    let r#impl = Impl::new(&input.ident, &input.generics);

    let output = match &mut input.data {
        syn::Data::Enum(data) => Enum::parse(&derive, &context, &r#impl, data)
            .ok()
            .into_token_stream(),

        syn::Data::Struct(data) => Struct::parse(
            &derive,
            &context,
            &r#impl,
            &mut input.attrs,
            &mut data.fields,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => Union::parse(&derive, &context, &r#impl, data)
            .ok()
            .into_token_stream(),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(
            &derive,
            &input.ident,
            attrs.own_crate,
            output.into_token_stream(),
        )
    };

    let derived = quote! {
        #input
        #output
    };

    println!("{}", derived.to_string());
    derived
}

fn to_compile_errors(errors: Vec<syn::Error>) -> TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn wrap_in_const(
    derive: &DeriveKind,
    name: &syn::Ident,
    own_crate: bool,
    tokens: TokenStream,
) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_{}_FOR_{}",
        match derive {
            DeriveKind::Serialize => "SERIALIZE",
            DeriveKind::Deserialize => "DESERIALIZE",
            DeriveKind::State => "STATE",
        },
        string::to_snake_case(&name.to_string()).to_uppercase()
    );

    let (extern_crate, krate) = if own_crate {
        (quote!(), quote!(crate))
    } else {
        (quote! { extern crate steit; }, quote!(steit))
    };

    quote! {
        const #dummy_const: () = {
            #extern_crate
            use std::io::{self, Read};
            use #krate::{de, wire_type, Deserialize, Eof, Runtime, Runtimed, Serialize, WireType};
            #tokens
        };
    }
}
