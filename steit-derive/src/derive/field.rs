use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    ctx::Context,
};

use super::{
    derive::{self, DeriveSetting},
    variant::Variant,
};

struct FieldAttrs {
    tag: u16,
    tag_tokens: TokenStream,
    skip_state: bool,
}

impl FieldAttrs {
    pub fn parse(context: &Context, field: &mut syn::Field) -> derive::Result<Self> {
        let mut tag = Attr::new(context, "tag");
        let mut skip_state = Attr::new(context, "skip_state");

        (&mut field.attrs).parse(context, true, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,

            syn::Meta::Path(path) if skip_state.parse_path(path) => true,
            syn::Meta::NameValue(meta) if skip_state.parse_bool(meta) => true,

            _ => false,
        });

        if let Some((tag, tag_tokens)) = tag.get_with_tokens() {
            Ok(Self {
                tag,
                tag_tokens,
                skip_state: skip_state.get().unwrap_or_default(),
            })
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

    fn state(&self) -> bool {
        self.setting.runtime() && !self.attrs.skip_state
    }

    pub fn access(&self) -> TokenStream {
        access(&self.name, self.index)
    }

    pub fn init(&self) -> TokenStream {
        let tag = self.tag();

        let value = match self.state() {
            true => quote!(State::with_runtime(runtime.nested(#tag))),
            false => quote!(Default::default()),
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
        let field_name = match &self.name {
            Some(name) => name.clone(),
            None => format_ident!("f_{}", self.index),
        };

        let setter_name = match variant {
            Some(variant) => format_ident!("set_{}_{}", variant.snake_case_name(), field_name),
            None => format_ident!("set_{}", field_name),
        };

        let setter_with_name = format_ident!("{}_with", setter_name);

        let ty = &self.ty;
        let tag = self.attrs.tag;
        let destructure = self.destructure();
        let field = self.field(variant.is_some());

        let (reset_variant, setter) = match variant {
            Some(variant) => {
                let qual = variant.qual();
                let ctor_name = variant.ctor_name();

                let (new_variant, log_update) = if self.state() {
                    (
                        quote!(Self::#ctor_name(self.runtime().parent())),
                        quote! { value.runtime().parent().log_update_in_place(&value).unwrap(); },
                    )
                } else {
                    (quote!(Self::#ctor_name()), quote!())
                };

                (
                    quote! {
                        if let #struct_name #qual { .. } = self {
                        } else {
                            let value = #new_variant;
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

        let (setter, setter_with) = if self.state() {
            let (assign_runtime, log_update) = (
                quote! { let runtime = self.runtime(); },
                quote! { runtime.log_update(#tag, &value).unwrap(); },
            );

            (
                quote! {
                    pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                        #reset_variant
                        #assign_runtime
                        #log_update
                        #setter
                        self
                    }
                },
                quote! {
                    pub fn #setter_with_name(&mut self, f: impl FnOnce(Runtime) -> #ty) -> &mut Self {
                        #reset_variant
                        #assign_runtime
                        runtime.pause_logger();
                        let value = f(runtime.nested(#tag));
                        runtime.unpause_logger();
                        #log_update
                        #setter
                        self
                    }
                },
            )
        } else {
            (
                quote! {
                    pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                        #reset_variant
                        #setter
                        self
                    }
                },
                quote!(),
            )
        };

        quote! {
            #setter
            #setter_with
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

    pub fn runtime_setter(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);

        if self.state() {
            quote! { #field.set_runtime(runtime.nested(#tag)); }
        } else {
            quote!()
        }
    }

    pub fn replayer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);

        if self.state() {
            quote!(#tag => #field.handle(path, kind, reader))
        } else {
            quote!(#tag => Ok(()))
        }
    }

    pub fn meta(&self) -> TokenStream {
        let name = self.alias().to_string();

        let ty = &self.ty;
        let type_name = &*quote!(#ty).to_string();

        let ty = match type_name {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "bool" => {
                quote!(&FieldType::Primitive(#type_name))
            }

            _ => quote!(<#ty as IsFieldType>::FIELD_TYPE),
        };

        let tag = self.tag();

        quote! {
            Field {
                name: #name,
                ty: #ty,
                tag: #tag,
            }
        }
    }
}

pub struct ExtraField {
    name: Option<syn::Ident>,
    ty: syn::Type,
    index: usize,
}

impl ExtraField {
    pub fn new(name: impl Into<Option<syn::Ident>>, ty: syn::Type, index: usize) -> Self {
        Self {
            name: name.into(),
            ty,
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
        access(&self.name, self.index)
    }

    pub fn init(&self, value: TokenStream) -> TokenStream {
        let access = self.access();

        if access.to_string() != value.to_string() {
            init(self.access(), value)
        } else {
            value
        }
    }

    pub fn destructure(&self, name: TokenStream) -> TokenStream {
        self.init(name)
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
