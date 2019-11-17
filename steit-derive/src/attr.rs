use std::{fmt, str::FromStr};

use proc_macro2::TokenStream;
use quote::ToTokens;

use super::ctx::Context;

pub struct Attr<'a, T> {
    context: &'a Context,
    name: &'static str,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'a, T> Attr<'a, T> {
    pub fn new(context: &'a Context, name: &'static str) -> Self {
        Self {
            context,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    pub fn set(&mut self, tokens: impl ToTokens, value: T) {
        if self.value.is_some() {
            self.context
                .error(tokens, format!("duplicate steit attribute `{}`", self.name));
        } else {
            self.tokens = tokens.to_token_stream();
            self.value = Some(value);
        }
    }

    pub fn get(self) -> Option<T> {
        self.value
    }

    pub fn get_with_tokens(self) -> Option<(T, TokenStream)> {
        match self.value {
            Some(value) => Some((value, self.tokens)),
            None => None,
        }
    }

    pub fn parse_name_value(
        &mut self,
        meta: &syn::MetaNameValue,
        mut f: impl FnMut(&syn::Lit) -> Result<T, &str>,
    ) -> bool {
        if meta.path.is_ident(self.name) {
            match f(&meta.lit) {
                Ok(value) => self.set(meta, value),
                Err(ty) => self.context.error(
                    &meta.lit,
                    format!("expected `{}` attribute to be {}", self.name, ty),
                ),
            }

            true
        } else {
            false
        }
    }
}

impl Attr<'_, bool> {
    pub fn parse_path(&mut self, path: &syn::Path) -> bool {
        if path.is_ident(self.name) {
            self.set(path, true);
            true
        } else {
            false
        }
    }

    pub fn parse_bool(&mut self, meta: &syn::MetaNameValue) -> bool {
        self.parse_name_value(meta, |lit| match lit {
            syn::Lit::Bool(lit) => Ok(lit.value),
            _ => Err("a boolean"),
        })
    }
}

impl<T> Attr<'_, T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    pub fn parse_int(&mut self, meta: &syn::MetaNameValue) -> bool {
        self.parse_name_value(meta, |lit| match lit {
            syn::Lit::Int(lit) => lit.base10_parse().map_err(|_| "an integer"),
            _ => Err("an integer"),
        })
    }
}

impl Attr<'_, String> {
    pub fn parse_str(&mut self, meta: &syn::MetaNameValue) -> bool {
        self.parse_name_value(meta, |lit| match lit {
            syn::Lit::Str(lit) => Ok(lit.value()),
            _ => Err("a string"),
        })
    }
}

pub struct VecAttr<'a, T> {
    context: &'a Context,
    name: &'static str,
    values: Vec<T>,
}

impl<'a, T> VecAttr<'a, T> {
    pub fn new(context: &'a Context, name: &'static str) -> Self {
        Self {
            context,
            name,
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn get(self) -> Vec<T> {
        self.values
    }

    pub fn parse_list(
        &mut self,
        meta: &syn::MetaList,
        mut f: impl FnMut(&syn::Lit) -> Result<T, &str>,
    ) -> bool {
        if meta.path.is_ident(self.name) {
            for meta in &meta.nested {
                match meta {
                    syn::NestedMeta::Lit(lit) => match f(lit) {
                        Ok(value) => self.insert(value),
                        Err(ty) => self.context.error(
                            lit,
                            format!("expected `{}` attribute to be a list of {}", self.name, ty),
                        ),
                    },
                    _ => self.context.error(
                        meta,
                        format!("expected `{}` attribute to be a literal", self.name),
                    ),
                }
            }

            true
        } else {
            false
        }
    }
}

impl<T> VecAttr<'_, T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    pub fn parse_int_list(&mut self, meta: &syn::MetaList) -> bool {
        self.parse_list(meta, |lit| match lit {
            syn::Lit::Int(lit) => lit.base10_parse().map_err(|_| "integer"),
            _ => Err("integer"),
        })
    }
}

pub trait AttrParse {
    fn parse(
        self,
        context: &Context,
        error_on_unknown: bool,
        f: &mut impl FnMut(&syn::Meta) -> bool,
    ) -> syn::AttributeArgs;
}

impl AttrParse for syn::AttributeArgs {
    fn parse(
        self,
        context: &Context,
        error_on_unknown: bool,
        f: &mut impl FnMut(&syn::Meta) -> bool,
    ) -> syn::AttributeArgs {
        let mut unknown = Vec::new();

        for meta in self {
            unknown.extend(parse_meta(meta, context, error_on_unknown, f));
        }

        unknown
    }
}

// Cannot merge this with the implementation above
// since that would cause a conflict with the one of `&mut Vec<syn::Attribute>`.
impl AttrParse for syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]> {
    fn parse(
        self,
        context: &Context,
        error_on_unknown: bool,
        f: &mut impl FnMut(&syn::Meta) -> bool,
    ) -> syn::AttributeArgs {
        let mut unknown = Vec::new();

        for meta in self {
            unknown.extend(parse_meta(meta, context, error_on_unknown, f));
        }

        unknown
    }
}

impl AttrParse for &mut Vec<syn::Attribute> {
    fn parse(
        self,
        context: &Context,
        error_on_unknown: bool,
        f: &mut impl FnMut(&syn::Meta) -> bool,
    ) -> syn::AttributeArgs {
        let mut unknown = Vec::new();

        // Retain only non-steit attributes
        self.retain(|attr| {
            if !attr.path.is_ident("steit") {
                return true;
            }

            let meta_list = match attr.parse_meta() {
                Ok(syn::Meta::List(meta)) => meta.nested,

                Ok(other) => {
                    context.error(other, "expected #[steit(...)]");
                    return false;
                }

                Err(error) => {
                    context.syn_error(error);
                    return false;
                }
            };

            unknown.extend(meta_list.parse(context, error_on_unknown, f));
            false
        });

        unknown
    }
}

fn parse_meta(
    meta: syn::NestedMeta,
    context: &Context,
    error_on_unknown: bool,
    f: &mut impl FnMut(&syn::Meta) -> bool,
) -> syn::AttributeArgs {
    let mut unknown = Vec::new();

    match meta {
        syn::NestedMeta::Meta(meta) if f(&meta) => (),

        syn::NestedMeta::Meta(item) => {
            if error_on_unknown {
                let path = item.path().to_token_stream().to_string().replace(' ', "");
                context.error(item.path(), format!("unknown steit attribute `{}`", path));
            }

            unknown.push(syn::NestedMeta::Meta(item));
        }

        syn::NestedMeta::Lit(lit) => {
            if error_on_unknown {
                context.error(&lit, "unexpected literal in steit attributes");
            }

            unknown.push(syn::NestedMeta::Lit(lit));
        }
    }

    unknown
}
