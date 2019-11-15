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

pub struct DeriveSetting {
    serialize: bool,
    deserialize: bool,
    merge: bool,
    state: bool,

    own_crate: bool,
    no_cached_size: bool,
}

impl DeriveSetting {
    pub fn parse(context: &Context, args: syn::AttributeArgs) -> Self {
        let mut serialize = Attr::new(context, "Serialize");
        let mut merge = Attr::new(context, "Merge");
        let mut deserialize = Attr::new(context, "Deserialize");
        let mut state = Attr::new(context, "State");

        let mut own_crate = Attr::new(context, "own_crate");
        let mut no_cached_size = Attr::new(context, "no_cached_size");

        args.parse(context, true, &mut |meta| match meta {
            syn::Meta::Path(path) if serialize.parse_path(path) => true,
            syn::Meta::Path(path) if merge.parse_path(path) => true,
            syn::Meta::Path(path) if deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if state.parse_path(path) => true,

            syn::Meta::Path(path) if own_crate.parse_path(path) => true,
            syn::Meta::NameValue(meta) if own_crate.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_cached_size.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_cached_size.parse_bool(meta) => true,

            _ => false,
        });

        let serialize = serialize.get().unwrap_or_default();
        let merge = merge.get().unwrap_or_default();
        let deserialize = deserialize.get().unwrap_or_default();
        let state = state.get().unwrap_or_default();

        Self {
            serialize: serialize || state,
            merge: merge || deserialize || state,
            deserialize: deserialize || state,
            state,

            own_crate: own_crate.get().unwrap_or_default(),
            no_cached_size: no_cached_size.get().unwrap_or_default(),
        }
    }

    pub fn extern_crate(&self) -> TokenStream {
        match self.own_crate {
            true => quote!(),
            false => quote! { extern crate steit; },
        }
    }

    pub fn krate(&self) -> TokenStream {
        match self.own_crate {
            true => quote!(crate),
            false => quote!(steit),
        }
    }

    pub fn ctors(&self, is_enum: bool) -> bool {
        if is_enum {
            self.merge
        } else {
            self.deserialize
        }
    }

    pub fn setters(&self) -> bool {
        self.state
    }

    pub fn default(&self, is_enum: bool) -> bool {
        if is_enum {
            self.merge
        } else {
            self.deserialize
        }
    }

    pub fn runtime(&self) -> bool {
        self.state
    }
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let context = Context::new();
    let setting = DeriveSetting::parse(&context, args);
    let r#impl = Impl::new(&input.ident, &input.generics);

    let output = match &mut input.data {
        syn::Data::Enum(data) => Enum::parse(&setting, &context, &r#impl, data)
            .ok()
            .into_token_stream(),

        syn::Data::Struct(data) => Struct::parse(
            &setting,
            &context,
            &r#impl,
            &mut input.attrs,
            &mut data.fields,
            None,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => Union::parse(&setting, &context, &r#impl, data)
            .ok()
            .into_token_stream(),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(&setting, &input.ident, output.into_token_stream())
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

fn wrap_in_const(setting: &DeriveSetting, name: &syn::Ident, tokens: TokenStream) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_STEIT_FOR_{}",
        string::to_snake_case(&name.to_string()).to_uppercase()
    );

    let extern_crate = setting.extern_crate();
    let krate = setting.krate();

    quote! {
        const #dummy_const: () = {
            #extern_crate

            use std::io::{self, Read};

            use #krate::{
                exhaust_nested,
                wire_type::{self, WireType},
                Deserialize,
                Eof,
                Merge,
                ReplayKind,
                Runtime,
                Runtimed,
                Serialize,
                State,
            };

            #tokens
        };
    }
}
