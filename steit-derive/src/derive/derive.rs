use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attribute, AttributeParse},
    ctx::Context,
    r#impl::Implementer,
    str_util,
};

use super::{r#enum::Enum, r#struct::Struct};

pub type Result<T> = std::result::Result<T, ()>;

pub struct DeriveSetting {
    pub derive_serialize: bool,
    pub derive_deserialize: bool,
    pub derive_state: bool,

    pub derive_partial_eq: bool,
    pub derive_default: bool,
    pub derive_hash: bool,

    pub derives: syn::AttributeArgs,

    pub steit_owned: bool,

    pub has_size_cache: bool,
    pub has_runtime: bool,

    pub derive_ctors: bool,
    pub derive_setters: bool,
    pub derive_wire_type: bool,
    pub derive_meta: bool,

    pub ctor_prefix: String,
    pub size_cache_renamed: Option<(String, TokenStream)>,
    pub runtime_renamed: Option<(String, TokenStream)>,
}

impl DeriveSetting {
    pub fn parse(
        ctx: &Context,
        args: syn::AttributeArgs,
        attrs: &mut Vec<syn::Attribute>,
    ) -> (Self, syn::AttributeArgs) {
        // Arguments

        let mut derive_serialize = Attribute::new(ctx, "Serialize");
        let mut derive_deserialize = Attribute::new(ctx, "Deserialize");
        let mut derive_state = Attribute::new(ctx, "State");

        let mut derive_partial_eq = Attribute::new(ctx, "PartialEq");
        let mut derive_default = Attribute::new(ctx, "Default");
        let mut derive_hash = Attribute::new(ctx, "Hash");

        let derives = args.parse(ctx, false, |meta| match meta {
            syn::Meta::Path(path) if derive_serialize.parse_path(path) => true,
            syn::Meta::Path(path) if derive_deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if derive_state.parse_path(path) => true,

            syn::Meta::Path(path) if derive_partial_eq.parse_path(path) => true,
            syn::Meta::Path(path) if derive_default.parse_path(path) => true,
            syn::Meta::Path(path) if derive_hash.parse_path(path) => true,

            _ => false,
        });

        let derive_state = derive_state.get().unwrap_or_default();
        let derive_serialize = derive_state || derive_serialize.get().unwrap_or_default();
        let derive_deserialize = derive_state || derive_deserialize.get().unwrap_or_default();

        let derive_default = derive_deserialize || derive_default.get().unwrap_or_default();

        // Attributes

        let mut steit_owned = Attribute::new(ctx, "steit_owned");

        let mut no_size_cache = Attribute::new(ctx, "no_size_cache");

        let mut derive_ctors = Attribute::new(ctx, "derive_ctors");
        let mut derive_setters = Attribute::new(ctx, "derive_setters");
        let mut no_meta = Attribute::new(ctx, "no_meta");

        let mut ctor_prefix = Attribute::new(ctx, "ctor_prefix");
        let mut size_cache_renamed = Attribute::new(ctx, "size_cache_renamed");
        let mut runtime_renamed = Attribute::new(ctx, "runtime_renamed");

        let unknown_attrs = attrs.parse(ctx, false, |meta| match meta {
            syn::Meta::Path(path) if steit_owned.parse_path(path) => true,
            syn::Meta::NameValue(meta) if steit_owned.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::Path(path) if derive_ctors.parse_path(path) => true,
            syn::Meta::NameValue(meta) if derive_ctors.parse_bool(meta) => true,

            syn::Meta::Path(path) if derive_setters.parse_path(path) => true,
            syn::Meta::NameValue(meta) if derive_setters.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_meta.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_meta.parse_bool(meta) => true,

            syn::Meta::NameValue(path) if ctor_prefix.parse_str(path) => true,
            syn::Meta::NameValue(meta) if size_cache_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            _ => false,
        });

        let has_size_cache = derive_serialize && !no_size_cache.get().unwrap_or_default();
        let has_runtime = derive_state;

        let derive_ctors = derive_deserialize || derive_ctors.get().unwrap_or_default();
        let derive_setters = derive_state || derive_setters.get().unwrap_or_default();
        let derive_wire_type = derive_serialize || derive_deserialize;
        let derive_meta = derive_deserialize && !no_meta.get().unwrap_or_default();

        (
            Self {
                derive_serialize,
                derive_deserialize,
                derive_state,

                derive_partial_eq: derive_partial_eq.get().unwrap_or_default(),
                derive_default,
                derive_hash: derive_hash.get().unwrap_or_default(),

                derives,

                steit_owned: steit_owned.get().unwrap_or_default(),

                has_size_cache,
                has_runtime,

                derive_ctors,
                derive_setters,
                derive_wire_type,
                derive_meta,

                ctor_prefix: ctor_prefix.get().unwrap_or_else(|| "new".to_string()),
                size_cache_renamed: size_cache_renamed.get_with_tokens(),
                runtime_renamed: runtime_renamed.get_with_tokens(),
            },
            unknown_attrs,
        )
    }

    pub fn extern_crate(&self) -> Option<TokenStream> {
        if !self.steit_owned {
            Some(quote! { extern crate steit; })
        } else {
            None
        }
    }

    pub fn krate(&self) -> TokenStream {
        if self.steit_owned {
            quote!(crate)
        } else {
            quote!(steit)
        }
    }
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let ctx = Context::new();
    let impler = Implementer::new(&input.ident, &input.generics);
    let (setting, unknown_attrs) = DeriveSetting::parse(&ctx, args, &mut input.attrs);
    let type_params = parse_type_params(&ctx, &input.generics);

    let output = match &mut input.data {
        syn::Data::Struct(data) => Struct::parse(
            &ctx,
            &impler,
            &setting,
            unknown_attrs,
            &type_params,
            &mut data.fields,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Enum(data) => Enum::parse(
            &ctx,
            &impler,
            &setting,
            unknown_attrs,
            &type_params,
            &mut data.variants,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => {
            ctx.error(data.union_token, "unions are not supported");
            quote!()
        }
    };

    let output = wrap_in_const(&setting, &input.ident, output);
    let derives = setting.derives;
    let errors = ctx.check().err().map(to_compile_errors);

    let derived = quote! {
        #[derive(#(#derives),*)]
        #input
        #output
        #errors
    };

    // println!("{}", derived);
    derived
}

fn parse_type_params<'a>(ctx: &Context, generics: &'a syn::Generics) -> Vec<&'a syn::TypeParam> {
    let mut type_params = Vec::new();

    for generic in &generics.params {
        match generic {
            syn::GenericParam::Type(ty) => type_params.push(ty),
            syn::GenericParam::Lifetime(_) => (),
            syn::GenericParam::Const(r#const) => {
                ctx.error(r#const, "const generics are not supported")
            }
        }
    }

    type_params
}

fn to_compile_errors(errors: Vec<syn::Error>) -> TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn wrap_in_const(setting: &DeriveSetting, name: &syn::Ident, tokens: TokenStream) -> TokenStream {
    let dummy_const = format_ident!(
        "_IMPL_STEIT_FOR_{}",
        str_util::to_snake_case(name.to_string()).to_uppercase(),
    );

    let extern_crate = setting.extern_crate();
    let krate = setting.krate();

    quote! {
        const #dummy_const: () = {
            #extern_crate

            use std::{
                hash::{Hash, Hasher},
                io::{self, Read},
            };

            use #krate::{
                de::{Deserialize, Reader},
                log::LogEntryKind,
                meta::*,
                rt::{Runtime, SizeCache},
                ser::Serialize,
                state::State,
                wire_fmt::{HasWireType, WireType},
            };

            #tokens
        };
    }
}
