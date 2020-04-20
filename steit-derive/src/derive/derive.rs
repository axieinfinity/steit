use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    context::Context,
    impler::Impler,
    string_util,
};

use super::{r#enum::Enum, r#struct::Struct, union::Union};

pub type Result<T> = std::result::Result<T, ()>;

pub struct DeriveSetting {
    pub serialize: bool,
    pub merge: bool,
    pub deserialize: bool,
    pub state: bool,

    pub own_crate: bool,

    pub no_ctors: bool,
    pub no_ctors_tokens: Option<TokenStream>,

    pub no_setters: bool,
    pub no_setters_tokens: Option<TokenStream>,

    pub no_size_cache: bool,
    pub no_meta: bool,
}

impl DeriveSetting {
    pub fn parse(context: &Context, args: syn::AttributeArgs) -> Self {
        let mut serialize = Attr::new(context, "Serialize");
        let mut merge = Attr::new(context, "Merge");
        let mut deserialize = Attr::new(context, "Deserialize");
        let mut state = Attr::new(context, "State");

        let mut own_crate = Attr::new(context, "own_crate");
        let mut no_ctors = Attr::new(context, "no_ctors");
        let mut no_setters = Attr::new(context, "no_setters");
        let mut no_size_cache = Attr::new(context, "no_size_cache");
        let mut no_meta = Attr::new(context, "no_meta");

        args.parse(context, true, |meta| match meta {
            syn::Meta::Path(path) if serialize.parse_path(path) => true,
            syn::Meta::Path(path) if merge.parse_path(path) => true,
            syn::Meta::Path(path) if deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if state.parse_path(path) => true,

            syn::Meta::Path(path) if own_crate.parse_path(path) => true,
            syn::Meta::NameValue(meta) if own_crate.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_ctors.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_ctors.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_setters.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_setters.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_meta.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_meta.parse_bool(meta) => true,

            _ => false,
        });

        let serialize = serialize.get().unwrap_or_default();
        let merge = merge.get().unwrap_or_default();
        let deserialize = deserialize.get().unwrap_or_default();
        let state = state.get().unwrap_or_default();

        let (no_ctors, no_ctors_tokens) = match no_ctors.get_with_tokens() {
            Some((no_ctors, no_ctors_tokens)) => (no_ctors, Some(no_ctors_tokens)),
            None => (Default::default(), Default::default()),
        };

        let (no_setters, no_setters_tokens) = match no_setters.get_with_tokens() {
            Some((no_setters, no_setters_tokens)) => (no_setters, Some(no_setters_tokens)),
            None => (Default::default(), Default::default()),
        };

        Self {
            serialize: serialize || state,
            merge: merge || deserialize || state,
            deserialize: deserialize || state,
            state,

            own_crate: own_crate.get().unwrap_or_default(),

            no_ctors,
            no_ctors_tokens,

            no_setters,
            no_setters_tokens,

            no_size_cache: no_size_cache.get().unwrap_or_default(),
            no_meta: no_meta.get().unwrap_or_default(),
        }
    }

    pub fn extern_crate(&self) -> TokenStream {
        if self.own_crate {
            quote!()
        } else {
            quote! { extern crate steit; }
        }
    }

    pub fn krate(&self) -> TokenStream {
        if self.own_crate {
            quote!(crate)
        } else {
            quote!(steit)
        }
    }

    pub fn ctors(&self, context: &Context, is_enum: bool) -> bool {
        let force_ctors = if is_enum {
            self.merge
        } else {
            self.deserialize
        };

        if force_ctors && self.no_ctors {
            context.error(
                self.no_ctors_tokens.as_ref().unwrap(),
                "constructors are required",
            );

            return true;
        }

        if is_enum && self.setters(context) && self.no_ctors {
            context.error(
                self.no_ctors_tokens.as_ref().unwrap(),
                "enum constructors are required to generate setters",
            );

            return true;
        }

        !self.no_ctors
    }

    pub fn setters(&self, context: &Context) -> bool {
        let force_setters = self.state;

        if force_setters && self.no_setters {
            context.error(
                self.no_setters_tokens.as_ref().unwrap(),
                "setters are required",
            );
            return true;
        }

        !self.no_setters
    }

    pub fn default(&self) -> bool {
        self.deserialize
    }

    pub fn size_cache(&self) -> bool {
        !self.no_size_cache
    }

    pub fn runtime(&self) -> bool {
        self.state
    }

    pub fn meta(&self) -> bool {
        self.deserialize && !self.no_meta
    }
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let context = Context::new();
    let setting = DeriveSetting::parse(&context, args);
    let impler = Impler::new(&input.ident, &input.generics);

    let output = match &mut input.data {
        syn::Data::Enum(data) => Enum::parse(&setting, &context, &impler, &mut input.attrs, data)
            .ok()
            .into_token_stream(),

        syn::Data::Struct(data) => Struct::parse(
            &setting,
            &context,
            &impler,
            &mut input.attrs,
            &mut data.fields,
            None,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => Union::parse(&setting, &context, &impler, data)
            .ok()
            .into_token_stream(),
    };

    let output = wrap_in_const(&setting, &input.ident, output);
    let errors = context.check().err().map(to_compile_errors);

    /* let derived = */
    quote! {
        #input
        #output
        #errors
    } /* ;

      println!("{}", derived.to_string());
      derived */
}

fn to_compile_errors(errors: Vec<syn::Error>) -> TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn wrap_in_const(setting: &DeriveSetting, name: &syn::Ident, tokens: TokenStream) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_STEIT_FOR_{}",
        string_util::to_snake_case(&name.to_string()).to_uppercase()
    );

    let extern_crate = setting.extern_crate();
    let krate = setting.krate();

    quote! {
        const #dummy_const: () = {
            #extern_crate

            use std::io::{self, Read};

            use #krate::{
                exhaust_nested,
                gen::*,
                wire_type::{self, WireType},
                SizeCache,
                Deserialize,
                Eof,
                Merge,
                ReplayKind,
                Runtime,
                Serialize,
                State,
            };

            #tokens
        };
    }
}
