use std::collections::HashSet;

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

#[derive(PartialEq, Eq, Hash)]
pub enum DeriveKind {
    Serialize,
    Merge,
    Deserialize,
    State,
}

struct DeriveAttrs {
    serialize: bool,
    deserialize: bool,
    merge: bool,
    state: bool,

    no_cached_size: bool,
    own_crate: bool,
}

impl DeriveAttrs {
    pub fn parse(context: &Context, args: syn::AttributeArgs) -> Self {
        let mut serialize = Attr::new(context, "Serialize");
        let mut merge = Attr::new(context, "Merge");
        let mut deserialize = Attr::new(context, "Deserialize");
        let mut state = Attr::new(context, "State");

        let mut no_cached_size = Attr::new(context, "no_cached_size");
        let mut own_crate = Attr::new(context, "own_crate");

        args.parse(context, true, &mut |meta| match meta {
            syn::Meta::Path(path) if serialize.parse_path(path) => true,
            syn::Meta::Path(path) if merge.parse_path(path) => true,
            syn::Meta::Path(path) if deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if state.parse_path(path) => true,

            syn::Meta::Path(path) if no_cached_size.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_cached_size.parse_bool(meta) => true,

            syn::Meta::Path(path) if own_crate.parse_path(path) => true,
            syn::Meta::NameValue(meta) if own_crate.parse_bool(meta) => true,

            _ => false,
        });

        Self {
            serialize: serialize.get().unwrap_or_default(),
            merge: merge.get().unwrap_or_default(),
            deserialize: deserialize.get().unwrap_or_default(),
            state: state.get().unwrap_or_default(),

            no_cached_size: no_cached_size.get().unwrap_or_default(),
            own_crate: own_crate.get().unwrap_or_default(),
        }
    }
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let context = Context::new();
    let attrs = DeriveAttrs::parse(&context, args);
    let r#impl = Impl::new(&input.ident, &input.generics);

    let mut derives = HashSet::new();

    if attrs.serialize {
        derives.insert(DeriveKind::Serialize);
    }

    if attrs.merge {
        derives.insert(DeriveKind::Merge);
    }

    if attrs.deserialize {
        derives.insert(DeriveKind::Merge);
        derives.insert(DeriveKind::Deserialize);
    }

    if attrs.state {
        derives.insert(DeriveKind::Serialize);
        derives.insert(DeriveKind::Merge);
        derives.insert(DeriveKind::Deserialize);
    }

    let output = match &mut input.data {
        syn::Data::Enum(data) => Enum::parse(&derives, &context, &r#impl, data)
            .ok()
            .into_token_stream(),

        syn::Data::Struct(data) => Struct::parse(
            &derives,
            &context,
            &r#impl,
            &mut input.attrs,
            &mut data.fields,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => Union::parse(&derives, &context, &r#impl, data)
            .ok()
            .into_token_stream(),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(&input.ident, attrs.own_crate, output.into_token_stream())
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

fn wrap_in_const(name: &syn::Ident, own_crate: bool, tokens: TokenStream) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_STEIT_FOR_{}",
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
            use #krate::{de, wire_type, Deserialize, Eof, Merge, Runtime, Runtimed, Serialize, WireType};
            #tokens
        };
    }
}
