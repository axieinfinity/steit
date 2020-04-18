use proc_macro2::TokenStream;

use crate::{
    attr::{Attr, AttrParse},
    context::Context,
    impl_util::ImplUtil,
    string_util,
};

pub struct DeriveSetting {
    pub serialize: bool,
    pub merge: bool,
    pub deserialize: bool,
    pub state: bool,

    pub derives: syn::AttributeArgs,

    pub steit_owned: bool,
    pub cached_size_renamed: Option<(String, TokenStream)>,
    pub runtime_renamed: Option<(String, TokenStream)>,

    pub unknown: syn::AttributeArgs,
}

impl DeriveSetting {
    pub fn parse(
        context: &Context,
        args: syn::AttributeArgs,
        attrs: &mut Vec<syn::Attribute>,
    ) -> Self {
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

        let mut steit_owned = Attr::new(context, "steit_owned");
        let mut cached_size_renamed = Attr::new(context, "cached_size_renamed");
        let mut runtime_renamed = Attr::new(context, "runtime_renamed");

        let unknown = attrs.parse(context, false, |meta| match meta {
            syn::Meta::Path(path) if steit_owned.parse_path(path) => true,
            syn::Meta::NameValue(meta) if steit_owned.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if cached_size_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            _ => false,
        });

        Self {
            serialize: serialize.get().unwrap_or_default(),
            merge: merge.get().unwrap_or_default(),
            deserialize: deserialize.get().unwrap_or_default(),
            state: state.get().unwrap_or_default(),

            derives,

            steit_owned: steit_owned.get().unwrap_or_default(),
            cached_size_renamed: cached_size_renamed.get_with_tokens(),
            runtime_renamed: runtime_renamed.get_with_tokens(),

            unknown,
        }
    }

    pub fn extern_crate(&self) -> TokenStream {
        if self.steit_owned {
            quote!()
        } else {
            quote! { extern crate steit; }
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
    let context = Context::new();
    let setting = DeriveSetting::parse(&context, args, &mut input.attrs);
    let impl_util = ImplUtil::new(&input.ident, &input.generics);

    let output = match &mut input.data {
        syn::Data::Enum(_data) => quote!(),
        syn::Data::Struct(_data) => quote!(),
        syn::Data::Union(_data) => quote!(),
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
                exhaust_nested,
                gen::*,
                wire_type::{self, WireType},
                CachedSize,
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
