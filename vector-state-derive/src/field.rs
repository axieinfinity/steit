use crate::attr::{Attr, AttrHolder};
use crate::context::Context;
use crate::util;

// Note that we intentionally exclude some unsupported primitive types
const PRIMITIVE_TYPES: &[&str] = &["bool", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"];

pub enum FieldKind {
    Primitive {
        default: Option<Attr<proc_macro2::TokenStream>>,
    },

    State,
}

pub struct IndexedField<'a> {
    name: Option<syn::Ident>,
    ty: &'a syn::Type,
    index: usize,
    tag: Attr<u16>,
    kind: FieldKind,
}

impl<'a> IndexedField<'a> {
    pub fn parse(context: &Context, field: &'a syn::Field, index: usize) -> Result<Self, ()> {
        let ty = &field.ty;
        let full_type_name = quote!(#ty).to_string();
        let is_primitive = PRIMITIVE_TYPES.contains(&&*full_type_name);

        Self::parse_attrs(context, &field, &field.attrs, is_primitive).map(|(tag, default)| {
            let kind = if is_primitive {
                FieldKind::Primitive { default }
            } else {
                FieldKind::State
            };

            Self {
                name: field.ident.clone(),
                ty,
                index,
                tag,
                kind,
            }
        })
    }

    fn parse_attrs(
        context: &Context,
        field: &syn::Field,
        attrs: &[syn::Attribute],
        is_primitive: bool,
    ) -> Result<(Attr<u16>, Option<Attr<proc_macro2::TokenStream>>), ()> {
        let mut tag_holder = AttrHolder::new(context, "tag");
        let mut default_holder = AttrHolder::new(context, "default");

        let mut tag_encountered = false;

        for item in attrs
            .iter()
            .flat_map(|attr| util::get_state_meta_items(context, attr))
            .flatten()
        {
            match &item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(item)) if item.path.is_ident("tag") => {
                    tag_encountered = true;

                    if let Ok(lit) = util::get_lit_int(context, "tag", &item.lit) {
                        if let Ok(tag) = lit.base10_parse() {
                            tag_holder.set(lit, tag);
                        } else {
                            context.error(lit, format!("unable to parse #[state(tag = {})]", lit));
                        }
                    }
                }

                syn::NestedMeta::Meta(syn::Meta::NameValue(item))
                    if item.path.is_ident("default") =>
                {
                    if !is_primitive {
                        context.error(item, "unexpected default value for this nested state");
                    }

                    if let Ok(lit) = util::get_lit_str(context, "default", &item.lit) {
                        if let Ok(default) = lit.value().parse() {
                            default_holder.set(lit, default);
                        } else {
                            context.error(
                                lit,
                                format!("unable to parse #[state(default = {:?})]", lit.value()),
                            );
                        }
                    }
                }

                syn::NestedMeta::Meta(item) => {
                    let path = item.path();
                    let path = quote!(path).to_string().replace(' ', "");
                    context.error(item.path(), format!("unknown state attribute `{}`", path));
                }

                syn::NestedMeta::Lit(lit) => {
                    context.error(lit, "unexpected literal in state attributes");
                }
            }
        }

        if let Some(tag) = tag_holder.attr() {
            Ok((tag, default_holder.attr()))
        } else {
            if !tag_encountered {
                context.error(field, "expected a `tag` attribute #[state(tag = ...)]");
            }

            Err(())
        }
    }

    pub fn tag(&self) -> &Attr<u16> {
        &self.tag
    }
}

pub struct PathField {
    index: usize,
}

impl PathField {
    pub fn new(_field: &syn::Field, index: usize) -> Self {
        Self { index }
    }
}
