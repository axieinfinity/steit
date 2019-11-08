use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::derive2::{
    attr::{Attr, AttrParse},
    ctx::Context,
    derive,
};

use super::DeriveKind;

struct FieldAttrs {
    tag: u16,
    tag_tokens: TokenStream,
}

impl FieldAttrs {
    pub fn parse(context: &Context, field: &mut syn::Field) -> derive::Result<Self> {
        let mut tag = Attr::new(context, "tag");

        (&mut field.attrs).parse(context, true, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,
            _ => false,
        });

        if let Some((tag, tag_tokens)) = tag.get_with_tokens() {
            Ok(Self { tag, tag_tokens })
        } else {
            context.error(field, "expected a valid tag #[steit(tag = ...)]");
            Err(())
        }
    }
}

pub struct Field<'a> {
    derive: &'a DeriveKind,
    name: Option<syn::Ident>,
    ty: syn::Type,
    index: usize,
    attrs: FieldAttrs,
}

impl<'a> Field<'a> {
    pub fn parse(
        derive: &'a DeriveKind,
        context: &Context,
        field: &mut syn::Field,
        index: usize,
    ) -> derive::Result<Self> {
        FieldAttrs::parse(context, field).map(|attrs| Self {
            derive,
            name: field.ident.clone(),
            ty: field.ty.clone(),
            index,
            attrs,
        })
    }

    pub fn tag(&self) -> u16 {
        self.attrs.tag
    }

    pub fn tag_with_tokens(&self) -> (u16, &TokenStream) {
        (self.attrs.tag, &self.attrs.tag_tokens)
    }

    pub fn access(&self) -> TokenStream {
        access(&self.name, self.index)
    }

    pub fn alias(&self) -> TokenStream {
        match &self.name {
            Some(name) => name.to_token_stream(),
            None => format_ident!("f_{}", self.index).to_token_stream(),
        }
    }

    pub fn destructure(&self) -> TokenStream {
        let access = self.access();

        match &self.name {
            Some(_) => access,
            None => {
                let alias = self.alias();
                quote!(#access: #alias)
            }
        }
    }

    pub fn field(&self, is_variant: bool) -> TokenStream {
        if is_variant {
            self.alias()
        } else {
            let access = self.access();
            quote!(self.#access)
        }
    }

    pub fn sizer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);
        quote! { size += #field.size_nested(#tag); }
    }

    pub fn serializer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);
        quote! { #field.serialize_nested(#tag, writer)?; }
    }

    pub fn merger(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);

        quote! {
            #tag if wire_type == #field.wire_type() => {
                #field.merge_nested(reader)?;
            }
        }
    }
}

pub struct Runtime {
    name: Option<syn::Ident>,
    index: usize,
}

impl Runtime {
    pub fn new(name: impl Into<Option<syn::Ident>>, index: usize) -> Self {
        Self {
            name: name.into(),
            index,
        }
    }

    pub fn declare(&self) -> syn::punctuated::Punctuated<syn::Field, syn::Token![,]> {
        if let Some(name) = &self.name {
            let fields: syn::FieldsNamed = syn::parse_quote!({ #name: Runtime2 });
            fields.named
        } else {
            let fields: syn::FieldsUnnamed = syn::parse_quote!(Runtime2);
            fields.unnamed
        }
    }
}

fn access(name: &Option<syn::Ident>, index: usize) -> TokenStream {
    match name {
        Some(name) => name.to_token_stream(),
        None => syn::Index::from(index).into_token_stream(),
    }
}
