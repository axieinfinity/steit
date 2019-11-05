use proc_macro2::TokenStream;

use crate::util;

use super::{
    attr::{self, Attr},
    ctx::Context,
};

pub enum Derive {
    Serialize,
    Deserialize,
    State,
}

#[derive(Debug)]
pub struct InputAttrs {
    own_crate: bool,
}

impl InputAttrs {
    pub fn parse(context: &Context, args: syn::AttributeArgs) -> Self {
        let mut own_crate = Attr::new(context, "own_crate");

        attr::parse_meta_list(context, args, &mut |meta| match meta {
            syn::Meta::Path(path) if own_crate.parse_path(path) => true,
            syn::Meta::NameValue(meta) if own_crate.parse_bool(meta) => true,
            _ => false,
        });

        Self {
            own_crate: own_crate.get().unwrap_or_default(),
        }
    }
}

pub fn derive(derive: Derive, args: syn::AttributeArgs, input: syn::DeriveInput) -> TokenStream {
    let context = Context::new();
    let attrs = InputAttrs::parse(&context, args);

    let output = quote!(#input);

    if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(&derive, &input.ident, attrs.own_crate, output)
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn wrap_in_const(
    derive: &Derive,
    name: &syn::Ident,
    own_crate: bool,
    tokens: TokenStream,
) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_{}_FOR_{}",
        match derive {
            Derive::Serialize => "SERIALIZE",
            Derive::Deserialize => "DESERIALIZE",
            Derive::State => "STATE",
        },
        util::to_snake_case(&name.to_string()).to_uppercase()
    );

    let (extern_crate, krate) = if own_crate {
        (quote!(), quote!(crate))
    } else {
        (quote!(extern crate steit), quote!(steit))
    };

    quote! {
        const #dummy_const: () = {
            #extern_crate;

            use std::io::{self, Read};

            // We don't import `Varint` directly
            // to avoid confusing `serialize` and `deserialize` calls.
            use #krate::{iowrap, varint, Deserialize, RawEntryKind, Runtime, Serialize};

            #tokens
        };
    }
}
