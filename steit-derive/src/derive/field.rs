use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    ctx::Context,
    derive,
};

use super::{variant::Variant, DeriveSetting};

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
    setting: &'a DeriveSetting,
    name: Option<syn::Ident>,
    ty: syn::Type,
    index: usize,
    attrs: FieldAttrs,
}

impl<'a> Field<'a> {
    pub fn parse(
        setting: &'a DeriveSetting,
        context: &Context,
        field: &mut syn::Field,
        index: usize,
    ) -> derive::Result<Self> {
        FieldAttrs::parse(context, field).map(|attrs| Self {
            setting,
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

    pub fn init(&self) -> TokenStream {
        let tag = self.tag();

        let value = match self.setting.no_runtime {
            false => quote!(Runtimed::with_runtime(runtime.nested(#tag))),
            true => quote!(Default::default()),
        };

        init(self.access(), value)
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

    pub fn setter(&self, struct_name: &syn::Ident, variant: Option<&Variant>) -> TokenStream {
        if self.setting.no_runtime {
            unreachable!("expected a `Runtime` field");
        }

        let field_name = match &self.name {
            Some(name) => name.clone(),
            None => format_ident!("f_{}", self.index),
        };

        let setter_name = match variant {
            Some(variant) => format_ident!("set_{}_{}", variant.snake_case_name(), field_name),
            None => format_ident!("set_{}", field_name),
        };

        let setter_name_with = format_ident!("{}_with", setter_name);

        let ty = &self.ty;
        let tag = self.attrs.tag;
        let destructure = self.destructure();
        let field = self.field(variant.is_some());

        let (reset_variant, setter) = match variant {
            Some(variant) => {
                let qual = variant.qual();
                let ctor_name = variant.ctor_name();

                let log_update = if self.setting.state {
                    quote! { value.runtime().parent().log_update_in_place(&value).unwrap(); }
                } else {
                    quote!()
                };

                (
                    quote! {
                        if let #struct_name #qual { .. } = self {
                        } else {
                            let value = Self::#ctor_name(self.runtime().parent());
                            #log_update
                            *self = value;
                        }
                    },
                    quote! {
                        if let #struct_name #qual { #destructure, .. } = self {
                            *#field = value;
                        }
                    },
                )
            }
            None => (quote!(), quote! { #field = value; }),
        };

        let log_update = if self.setting.state {
            quote! { runtime.log_update(#tag, &value).unwrap(); }
        } else {
            quote!()
        };

        quote! {
            pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                #reset_variant
                let runtime = self.runtime();
                #log_update
                #setter
                self
            }

            pub fn #setter_name_with(&mut self, f: impl FnOnce(Runtime) -> #ty) -> &mut Self {
                #reset_variant
                let runtime = self.runtime();
                let value = f(runtime.nested(#tag));
                #log_update
                #setter
                self
            }
        }
    }

    pub fn sizer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);
        quote! { size += #field.compute_size_nested(#tag); }
    }

    pub fn serializer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);
        quote! { #field.serialize_nested_with_cached_size(#tag, writer)?; }
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

    pub fn replayer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);
        quote!(#tag => #field.handle(path, kind, reader))
    }
}

pub struct Runtime<'a> {
    setting: &'a DeriveSetting,
    name: Option<syn::Ident>,
    index: usize,
}

impl<'a> Runtime<'a> {
    pub fn new(
        setting: &'a DeriveSetting,
        name: impl Into<Option<syn::Ident>>,
        index: usize,
    ) -> Self {
        Self {
            setting,
            name: name.into(),
            index,
        }
    }

    pub fn declare(&self) -> syn::punctuated::Punctuated<syn::Field, syn::Token![,]> {
        let krate = self.setting.krate();

        if let Some(name) = &self.name {
            let fields: syn::FieldsNamed = syn::parse_quote!({ #name: #krate::Runtime });
            fields.named
        } else {
            let fields: syn::FieldsUnnamed = syn::parse_quote!((#krate::Runtime));
            fields.unnamed
        }
    }

    pub fn access(&self) -> TokenStream {
        access(&self.name, self.index)
    }

    pub fn init(&self) -> TokenStream {
        let access = self.access();

        if &access.to_string() != "runtime" {
            init(self.access(), quote!(runtime))
        } else {
            quote!(runtime)
        }
    }
}

fn access(name: &Option<syn::Ident>, index: usize) -> TokenStream {
    match name {
        Some(name) => name.to_token_stream(),
        None => syn::Index::from(index).into_token_stream(),
    }
}

fn init(access: TokenStream, value: TokenStream) -> TokenStream {
    quote!(#access: #value)
}
