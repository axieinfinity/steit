use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    context::Context,
};

use super::{
    derive::{self, DeriveSetting},
    tag,
};

struct FieldAttrs {
    tag: u32,
    tag_tokens: TokenStream,
    no_state: bool,
    csharp_name: Option<String>,
}

impl FieldAttrs {
    pub fn parse(context: &Context, field: &mut syn::Field) -> derive::Result<Self> {
        let mut tag = Attr::new(context, "tag");
        let mut no_state = Attr::new(context, "no_state");
        let mut csharp_name = Attr::new(context, "csharp_name");

        (&mut field.attrs).parse(context, true, |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,

            syn::Meta::Path(path) if no_state.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_state.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if csharp_name.parse_str(meta) => true,

            _ => false,
        });

        let (tag, tag_tokens) = tag
            .get_with_tokens()
            .ok_or_else(|| context.error(field, "expected a valid tag #[steit(tag = …)]"))?;

        tag::validate(tag).map_err(|message| {
            context.error(&tag_tokens, message);
            ()
        })?;

        Ok(Self {
            tag,
            tag_tokens,
            no_state: no_state.get().unwrap_or_default(),
            csharp_name: csharp_name.get(),
        })
    }
}

pub struct Field {
    name: Option<syn::Ident>,
    ty: syn::Type,
    index: usize,
}

impl Field {
    pub fn new(name: Option<syn::Ident>, ty: syn::Type, index: usize) -> Self {
        Self { name, ty, index }
    }

    pub fn from_field(field: &syn::Field, index: usize) -> Self {
        Self {
            name: field.ident.clone(),
            ty: field.ty.clone(),
            index,
        }
    }

    pub fn declare(&self) -> syn::punctuated::Punctuated<syn::Field, syn::Token![,]> {
        let ty = &self.ty;

        if let Some(name) = &self.name {
            let fields: syn::FieldsNamed = syn::parse_quote!({ #name: #ty });
            fields.named
        } else {
            let fields: syn::FieldsUnnamed = syn::parse_quote!((#ty));
            fields.unnamed
        }
    }

    pub fn access(&self) -> TokenStream {
        match &self.name {
            Some(name) => name.to_token_stream(),
            None => syn::Index::from(self.index).into_token_stream(),
        }
    }

    pub fn init(&self, value: TokenStream) -> TokenStream {
        let access = self.access();

        if access.to_string() != value.to_string() {
            quote!(#access: #value)
        } else {
            value
        }
    }

    pub fn alias(&self) -> TokenStream {
        match &self.name {
            Some(name) => name.to_token_stream(),
            None => format_ident!("f{}", self.index).to_token_stream(),
        }
    }

    pub fn destructure(&self, name: TokenStream) -> TokenStream {
        self.init(name)
    }

    pub fn destructure_alias(&self) -> TokenStream {
        self.destructure(self.alias())
    }

    pub fn field(&self, is_variant: bool) -> TokenStream {
        if is_variant {
            self.alias()
        } else {
            let access = self.access();
            quote!(self.#access)
        }
    }
}

pub struct DeriveField<'a> {
    setting: &'a DeriveSetting,
    field: Field,
    attrs: FieldAttrs,
}

impl<'a> Deref for DeriveField<'a> {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DeriveField<'a> {
    pub fn parse(
        setting: &'a DeriveSetting,
        context: &Context,
        field: &mut syn::Field,
        index: usize,
    ) -> derive::Result<Self> {
        let attrs = FieldAttrs::parse(context, field)?;
        let field = Field::from_field(field, index);

        Ok(Self {
            setting,
            field,
            attrs,
        })
    }

    pub fn tag(&self) -> u32 {
        self.attrs.tag
    }

    pub fn tag_with_tokens(&self) -> (u32, &TokenStream) {
        (self.attrs.tag, &self.attrs.tag_tokens)
    }

    pub fn is_state(&self) -> bool {
        self.setting.state && !self.attrs.no_state
    }

    pub fn init_default(&self) -> TokenStream {
        self.init(if self.is_state() {
            let tag = self.tag();
            quote!(State::with_runtime(runtime.nested(#tag as u16)))
        } else {
            quote!(Default::default())
        })
    }

    pub fn sizer(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { size += #field.compute_size_nested_v2(#tag, true); }
    }

    pub fn serializer(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { #field.serialize_nested(#tag, true, writer)?; }
    }

    pub fn merger(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { #tag => #field.merge_nested_v2(wire_type, reader)? }
    }

    pub fn runtime_setter(&self, is_variant: bool) -> Option<TokenStream> {
        if self.is_state() {
            let tag = self.tag();
            let field = self.field(is_variant);
            Some(quote! { #field.set_runtime(runtime.nested(#tag as u16)); })
        } else {
            None
        }
    }
}
