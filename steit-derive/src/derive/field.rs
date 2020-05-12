use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attribute, AttributeParse},
    ctx::Context,
};

use super::{
    derive::{self, DeriveSetting},
    tag,
    variant::Variant,
};

struct FieldAttrs {
    tag: u32,
    tag_tokens: TokenStream,

    no_hash: bool,
    no_eq_hash: bool,
    no_state: bool,

    csharp_name: Option<String>,
}

impl FieldAttrs {
    pub fn parse(ctx: &Context, field: &mut syn::Field) -> derive::Result<Self> {
        let mut tag = Attribute::new(ctx, "tag");

        let mut no_hash = Attribute::new(ctx, "no_hash");
        let mut no_eq_hash = Attribute::new(ctx, "no_eq_hash");
        let mut no_state = Attribute::new(ctx, "no_state");

        let mut csharp_name = Attribute::new(ctx, "csharp_name");

        (&mut field.attrs).parse(ctx, true, |meta| match meta {
            syn::Meta::NameValue(meta) if tag.parse_int(meta) => true,

            syn::Meta::Path(path) if no_hash.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_hash.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_eq_hash.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_eq_hash.parse_bool(meta) => true,

            syn::Meta::Path(path) if no_state.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_state.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if csharp_name.parse_str(meta) => true,

            _ => false,
        });

        let (tag, tag_tokens) = tag
            .get_with_tokens()
            .ok_or_else(|| ctx.error(field, "expected a valid tag #[steit(tag = …)]"))?;

        tag::validate(tag).map_err(|message| {
            ctx.error(&tag_tokens, message);
        })?;

        Ok(Self {
            tag,
            tag_tokens,

            no_hash: no_hash.get().unwrap_or_default(),
            no_eq_hash: no_eq_hash.get().unwrap_or_default(),
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

    pub fn ty(&self) -> &syn::Type {
        &self.ty
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

    pub fn alias_prefixed(&self, prefix: impl Into<Option<syn::Ident>>) -> syn::Ident {
        let alias = match &self.name {
            Some(name) => format_ident!("{}", name),
            None => format_ident!("f{}", self.index),
        };

        match prefix.into() {
            Some(prefix) => format_ident!("{}_{}", prefix, alias),
            None => alias,
        }
    }

    pub fn alias(&self) -> syn::Ident {
        self.alias_prefixed(None)
    }

    pub fn destructure(&self, name: syn::Ident) -> TokenStream {
        self.init(name.into_token_stream())
    }

    pub fn destructure_alias(&self) -> TokenStream {
        self.destructure(self.alias())
    }

    pub fn destructure_alias_prefixed(&self, prefix: impl Into<Option<syn::Ident>>) -> TokenStream {
        self.destructure(self.alias_prefixed(prefix))
    }

    pub fn field_other(
        &self,
        other: impl Into<Option<syn::Ident>>,
        is_variant: bool,
    ) -> TokenStream {
        if is_variant {
            self.alias_prefixed(other).into_token_stream()
        } else {
            let other = other
                .into()
                .map_or(quote!(self), ToTokens::into_token_stream);
            let access = self.access();
            quote!(#other.#access)
        }
    }

    pub fn field(&self, is_variant: bool) -> TokenStream {
        self.field_other(None, is_variant)
    }
}

pub struct DeriveField<'a> {
    setting: &'a DeriveSetting,
    attrs: FieldAttrs,
    field: Field,
    type_meta: Option<TokenStream>,
}

impl<'a> Deref for DeriveField<'a> {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DeriveField<'a> {
    pub fn parse(
        ctx: &Context,
        setting: &'a DeriveSetting,
        type_params: &'a [&'a syn::TypeParam],
        field: &mut syn::Field,
        index: usize,
    ) -> derive::Result<Self> {
        let attrs = FieldAttrs::parse(ctx, field)?;
        let field = Field::from_field(field, index);

        let type_meta = if setting.has_meta() {
            Some(field_type_meta(ctx, &field.ty, type_params)?)
        } else {
            None
        };

        Ok(Self {
            setting,
            attrs,
            field,
            type_meta,
        })
    }

    pub fn tag(&self) -> u32 {
        self.attrs.tag
    }

    pub fn tag_with_tokens(&self) -> (u32, &TokenStream) {
        (self.attrs.tag, &self.attrs.tag_tokens)
    }

    pub fn is_state(&self) -> bool {
        self.setting.impl_state() && !self.attrs.no_state
    }

    pub fn init_default(&self) -> TokenStream {
        self.init(if self.is_state() {
            let tag = self.tag();
            quote!(State::with_runtime(runtime.nested(#tag)))
        } else {
            quote!(Default::default())
        })
    }

    pub fn setter(&self, struct_name: &syn::Ident, variant: Option<&Variant>) -> TokenStream {
        let setter_name = self.alias_prefixed(match variant {
            Some(variant) => format_ident!("set_{}", variant.snake_case_name()),
            None => format_ident!("set"),
        });

        let setter_with_name = format_ident!("{}_with", setter_name);

        let ty = &self.ty;
        let tag = self.tag();

        let (reset_variant, set_value) = if let Some(variant) = variant {
            let qual = variant.qual();
            let ctor_name = variant.ctor_name();

            let destructure = self.destructure(format_ident!("self_value"));

            let new_variant = if self.setting.impl_state() {
                quote! {{
                    let runtime = self.runtime().parent();
                    let value = Self::#ctor_name(runtime.clone());
                    runtime.log_update(&value).unwrap();
                    value
                }}
            } else {
                quote!(Self::#ctor_name())
            };

            (
                Some(quote! {
                    if let #struct_name #qual { .. } = self {
                    } else {
                        *self = #new_variant;
                    }
                }),
                quote! {
                    if let #struct_name #qual { #destructure, .. } = self {
                        *self_value = value;
                    }
                },
            )
        } else {
            let field = self.field(false);
            (None, quote! { #field = value; })
        };

        let (setter, setter_with) = if self.is_state() {
            let declare_runtime = quote! { let runtime = self.runtime(); };
            let log_update = quote! { runtime.log_update_child(#tag, &value).unwrap(); };

            (
                quote! {
                    pub fn #setter_name(&mut self, mut value: #ty) -> &mut Self {
                        #reset_variant
                        #declare_runtime
                        value.set_runtime(runtime.nested(#tag));
                        #log_update
                        #set_value
                        self
                    }
                },
                Some(quote! {
                    pub fn #setter_with_name(&mut self, get_value: impl FnOnce(Runtime) -> #ty) -> &mut Self {
                        #reset_variant
                        #declare_runtime
                        runtime.pause_logger();
                        let value = get_value(runtime.nested(#tag));
                        runtime.unpause_logger();
                        #log_update
                        #set_value
                        self
                    }
                }),
            )
        } else {
            (
                quote! {
                    pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                        #reset_variant
                        #set_value
                        self
                    }
                },
                None,
            )
        };

        quote! {
            #setter
            #setter_with
        }
    }

    pub fn eq(&self, is_variant: bool) -> Option<TokenStream> {
        if !self.attrs.no_eq_hash {
            let field = self.field(is_variant);
            let other_field = self.field_other(format_ident!("other"), is_variant);

            Some(quote! {
                if #field != #other_field {
                    return false;
                }
            })
        } else {
            None
        }
    }

    pub fn hash(&self, is_variant: bool) -> Option<TokenStream> {
        if !self.attrs.no_hash && !self.attrs.no_eq_hash {
            let field = self.field(is_variant);
            Some(quote! { #field.hash(state); })
        } else {
            None
        }
    }

    pub fn sizer(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { size += #field.compute_size_nested(#tag, true).unwrap(); }
    }

    pub fn serializer(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { #field.serialize_nested(#tag, true, writer)?; }
    }

    pub fn merger(&self, is_variant: bool) -> TokenStream {
        let tag = self.tag();
        let field = self.field(is_variant);
        quote! { #tag => #field.merge_nested(wire_type, reader)? }
    }

    pub fn runtime_setter(&self, is_variant: bool) -> Option<TokenStream> {
        if self.is_state() {
            let tag = self.tag();
            let field = self.field(is_variant);
            Some(quote! { #field.set_runtime(runtime.nested(#tag)); })
        } else {
            None
        }
    }

    pub fn replayer(&self, is_variant: bool) -> TokenStream {
        let tag = self.attrs.tag;
        let field = self.field(is_variant);

        if self.is_state() {
            quote!(#tag => #field.handle(path, kind, reader))
        } else {
            quote!(#tag => Ok(()))
        }
    }

    pub fn meta(&self) -> TokenStream {
        let rust_name = self.alias().to_string();
        let csharp_name = match &self.attrs.csharp_name {
            Some(csharp_name) => quote!(Some(#csharp_name)),
            None => quote!(None),
        };

        let type_meta = self.type_meta.as_ref().unwrap();
        let tag = self.tag();

        quote! {
            FieldMeta {
                name: &NameMeta {
                    rust: #rust_name,
                    csharp: #csharp_name,
                },
                ty: &#type_meta,
                tag: #tag,
            }
        }
    }
}

fn field_type_meta(
    ctx: &Context,
    ty: &syn::Type,
    type_params: &[&syn::TypeParam],
) -> derive::Result<TokenStream> {
    let type_not_supported = || {
        ctx.error(
            ty,
            concat!(
                "this type is not supported, ",
                "expected either paren `(T)`; ",
                "reference `&'a T`, `&'a mut T`; ",
                "or path type `a::b::T`",
            ),
        );

        Err(())
    };

    let is_type_param = |type_name: &str| {
        for type_param in type_params {
            if type_param.ident == type_name {
                return true;
            }
        }

        false
    };

    match ty {
        syn::Type::Paren(syn::TypeParen { elem, .. })
        | syn::Type::Reference(syn::TypeReference { elem, .. }) => {
            field_type_meta(&ctx, &*elem, type_params)
        }

        syn::Type::Path(syn::TypePath { qself, path }) => {
            if qself.is_some() {
                ctx.error(ty, "fully-qualified types are not supported");
                return Err(());
            }

            match &path.segments.last().unwrap().arguments {
                syn::PathArguments::None => {
                    if let Some(type_name) = path.get_ident() {
                        let type_name = type_name.to_string();

                        if is_type_param(&type_name) {
                            return Ok(quote!(FieldTypeMeta::TypeParam(#type_name)));
                        }
                    }

                    // This has primitive types covered.
                    Ok(quote!(FieldTypeMeta::Type(<#path as HasMeta>::TYPE)))
                }

                syn::PathArguments::AngleBracketed(args) => {
                    let mut arg_meta_list = Vec::new();

                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Lifetime(_) => (),

                            syn::GenericArgument::Type(ty) => {
                                let type_name = ty.to_token_stream().to_string();

                                let arg_meta = if is_type_param(&type_name) {
                                    quote!(FieldTypeMeta::TypeParam(#type_name))
                                } else {
                                    field_type_meta(ctx, ty, type_params)?
                                };

                                arg_meta_list.push(arg_meta);
                            }

                            syn::GenericArgument::Binding(_)
                            | syn::GenericArgument::Constraint(_)
                            | syn::GenericArgument::Const(_) => {
                                ctx.error(arg, "this kind of type argument is not supported");
                                return Err(());
                            }
                        }
                    }

                    Ok(quote!(FieldTypeMeta::Type(&TypeMeta::Ref(
                        <#path as HasMeta>::NAME,
                        &[#(#arg_meta_list,)*],
                    ))))
                }

                syn::PathArguments::Parenthesized(_) => type_not_supported(),
            }
        }

        _ => type_not_supported(),
    }
}
