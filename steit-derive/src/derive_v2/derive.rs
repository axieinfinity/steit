use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    context::Context,
    impler::Impler,
    string_util,
};

use super::{r#enum::Enum, r#struct::Struct};

pub type Result<T> = std::result::Result<T, ()>;

pub struct DeriveSetting {
    pub serialize: bool,
    pub merge: bool,
    pub deserialize: bool,
    pub state: bool,

    derives: syn::AttributeArgs,

    steit_owned: bool,

    no_ctors: bool,
    no_size_cache: bool,

    pub size_cache_renamed: Option<(String, TokenStream)>,
    pub runtime_renamed: Option<(String, TokenStream)>,
}

impl DeriveSetting {
    pub fn parse(
        context: &Context,
        is_struct: bool,
        args: syn::AttributeArgs,
        attrs: &mut Vec<syn::Attribute>,
    ) -> (Self, syn::AttributeArgs) {
        // Arguments

        let mut serialize = Attr::new(context, "Serialize");
        let mut merge = Attr::new(context, "Merge");
        let mut deserialize = Attr::new(context, "Deserialize");
        let mut state = Attr::new(context, "State");

        let derives = args.parse(context, false, |meta| match meta {
            syn::Meta::Path(path) if serialize.parse_path(path) => true,
            syn::Meta::Path(path) if merge.parse_path(path) => true,
            syn::Meta::Path(path) if deserialize.parse_path(path) => true,
            syn::Meta::Path(path) if state.parse_path(path) => true,
            _ => false,
        });

        let state = state.get().unwrap_or_default();
        let serialize = state || serialize.get().unwrap_or_default();
        let deserialize = state || deserialize.get().unwrap_or_default();
        let merge = deserialize || merge.get().unwrap_or_default();

        // Attributes

        let mut steit_owned = Attr::new(context, "steit_owned");

        let mut no_ctors = Attr::new(context, "no_ctors");
        let mut no_size_cache = Attr::new(context, "no_size_cache");

        let mut size_cache_renamed = Attr::new(context, "size_cache_renamed");
        let mut runtime_renamed = Attr::new(context, "runtime_renamed");

        let unknown_attrs = attrs.parse(context, false, |meta| match meta {
            syn::Meta::Path(path) if steit_owned.parse_path(path) => true,
            syn::Meta::NameValue(meta) if steit_owned.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_ctors.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_ctors.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if size_cache_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            _ => false,
        });

        let (mut no_ctors, no_ctors_tokens) = no_ctors.get_with_tokens().unwrap_or_default();
        let force_ctors = if is_struct { deserialize } else { merge };

        if force_ctors && no_ctors {
            context.error(
                no_ctors_tokens,
                if is_struct {
                    "constructor is required for `Deserialize` on struct"
                } else {
                    "constructors are required for `Merge` on enum"
                },
            );

            no_ctors = false;
        }

        (
            Self {
                serialize,
                merge,
                deserialize,
                state,

                derives,

                steit_owned: steit_owned.get().unwrap_or_default(),

                no_ctors,
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

    pub fn has_ctors(&self) -> bool {
        !self.no_ctors
    }

    pub fn has_size_cache(&self) -> bool {
        self.serialize && !self.no_size_cache
    }

    pub fn has_runtime(&self) -> bool {
        self.state
    }
}

pub fn derive(args: syn::AttributeArgs, mut input: syn::DeriveInput) -> TokenStream {
    let context = Context::new();
    let impler = Impler::new(&input.ident, &input.generics);

    let is_struct = if let syn::Data::Struct(_) = input.data {
        true
    } else {
        false
    };

    let (setting, unknown_attrs) =
        DeriveSetting::parse(&context, is_struct, args, &mut input.attrs);

    let output = match &mut input.data {
        syn::Data::Struct(data) => Struct::parse(
            &context,
            &impler,
            &setting,
            unknown_attrs,
            &mut data.fields,
            None,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Enum(data) => Enum::parse(
            &context,
            &impler,
            &setting,
            unknown_attrs,
            &mut data.variants,
        )
        .ok()
        .into_token_stream(),

        syn::Data::Union(data) => {
            context.error(data.union_token, "union is not supported");
            quote!()
        }
    };

    let output = wrap_in_const(&setting, &input.ident, output);
    let derives = setting.derives;
    let errors = context.check().err().map(to_compile_errors);

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
        string_util::to_snake_case(name.to_string()).to_uppercase(),
    );

    let extern_crate = setting.extern_crate();
    let krate = setting.krate();

    quote! {
        const #dummy_const: () = {
            #extern_crate

            use std::io::{self, Read};

            use #krate::{
                de_v2::{DeserializeV2, MergeNested, MergeV2, Reader},
                runtime::{Runtime, SizeCache},
                ser_v2::{SerializeNested, SerializeV2},
                wire_format::{HasWireType, WireTypeV2},
            };

            #tokens
        };
    }
}
