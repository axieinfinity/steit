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
    serialize: bool,
    deserialize: bool,
    state: bool,

    derives: syn::AttributeArgs,

    steit_owned: bool,

    no_size_cache: bool,

    pub size_cache_renamed: Option<(String, TokenStream)>,
    pub runtime_renamed: Option<(String, TokenStream)>,
}

macro_rules! getter {
    ($getter:ident -> $type:ty = _.$field:ident) => {
        pub fn $getter(&self) -> $type {
            self.$field
        }
    };
}

impl DeriveSetting {
    pub fn parse(
        ctx: &Context,
        args: syn::AttributeArgs,
        attrs: &mut Vec<syn::Attribute>,
    ) -> (Self, syn::AttributeArgs) {
        // Arguments

        let mut serialize = Attribute::new(ctx, "Serialize");
        let mut deserialize = Attribute::new(ctx, "Deserialize");
        let mut state = Attribute::new(ctx, "State");

        let derives = args.parse(ctx, false, |meta| match meta {
            syn::Meta::Path(path) if serialize.parse_path(path) => true,
            syn::Meta::Path(path) if deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if state.parse_path(path) => true,
            _ => false,
        });

        let state = state.get().unwrap_or_default();
        let serialize = state || serialize.get().unwrap_or_default();
        let deserialize = state || deserialize.get().unwrap_or_default();

        // Attributes

        let mut steit_owned = Attribute::new(ctx, "steit_owned");

        let mut no_size_cache = Attribute::new(ctx, "no_size_cache");

        let mut size_cache_renamed = Attribute::new(ctx, "size_cache_renamed");
        let mut runtime_renamed = Attribute::new(ctx, "runtime_renamed");

        let unknown_attrs = attrs.parse(ctx, false, |meta| match meta {
            syn::Meta::Path(path) if steit_owned.parse_path(path) => true,
            syn::Meta::NameValue(meta) if steit_owned.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if size_cache_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            _ => false,
        });

        (
            Self {
                serialize,
                deserialize,
                state,

                derives,

                steit_owned: steit_owned.get().unwrap_or_default(),

                no_size_cache: no_size_cache.get().unwrap_or_default(),

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

    getter!(impl_serialize -> bool = _.serialize);
    getter!(impl_deserialize -> bool = _.deserialize);
    getter!(impl_state -> bool = _.state);

    pub fn has_size_cache(&self) -> bool {
        self.serialize && !self.no_size_cache
    }

    getter!(has_runtime -> bool = _.state);
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let ctx = Context::new();
    let impler = Implementer::new(&input.ident, &input.generics);
    let (setting, unknown_attrs) = DeriveSetting::parse(&ctx, args, &mut input.attrs);

    let output = match &mut input.data {
        syn::Data::Struct(data) => Struct::parse(
            &ctx,
            &impler,
            &setting,
            unknown_attrs,
            &mut data.fields,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Enum(data) => {
            Enum::parse(&ctx, &impler, &setting, unknown_attrs, &mut data.variants)
                .ok()
                .into_token_stream()
        }

        syn::Data::Union(data) => {
            ctx.error(data.union_token, "union is not supported");
            quote!()
        }
    };

    let output = wrap_in_const(&setting, &input.ident, output);
    let derives = setting.derives;
    let errors = ctx.check().err().map(to_compile_errors);

    let derived = quote! {
        #[derive(#(#derives)*)]
        #input
        #output
        #errors
    };

    println!("{}", derived);
    derived
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

            use std::io::{self, Read};

            use #krate::{
                de_v2::{DeserializeV2, Reader},
                rt::{Runtime, SizeCache},
                ser_v2::SerializeV2,
                state_v2::StateV2,
                wire_fmt::{HasWireType, WireTypeV2},
            };

            #tokens
        };
    }
}
