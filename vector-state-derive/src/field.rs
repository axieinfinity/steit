use crate::{
    attr::{Attr, AttrValue},
    context::Context,
    util,
};

// Note that we intentionally exclude some unsupported primitive types
const PRIMITIVE_TYPES: &[&str] = &["bool", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"];

pub enum FieldKind {
    Primitive {
        default: Option<AttrValue<proc_macro2::TokenStream>>,
    },

    State,
}

pub struct IndexedField<'a> {
    name: Option<syn::Ident>,
    ty: &'a syn::Type,
    index: usize,
    tag: AttrValue<u16>,
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
    ) -> Result<(AttrValue<u16>, Option<AttrValue<proc_macro2::TokenStream>>), ()> {
        let mut tag_attr = Attr::new(context, "tag");
        let mut default_attr = Attr::new(context, "default");

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
                            tag_attr.set(lit, tag);
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
                            default_attr.set(lit, default);
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
                    let path = quote!(#path).to_string().replace(' ', "");
                    context.error(item.path(), format!("unknown state attribute `{}`", path));
                }

                syn::NestedMeta::Lit(lit) => {
                    context.error(lit, "unexpected literal in state attributes");
                }
            }
        }

        if let Some(tag) = tag_attr.value() {
            Ok((tag, default_attr.value()))
        } else {
            if !tag_encountered {
                context.error(field, "expected a `tag` attribute #[state(tag = ...)]");
            }

            Err(())
        }
    }

    pub fn tag(&self) -> &AttrValue<u16> {
        &self.tag
    }

    pub fn to_init(&self) -> proc_macro2::TokenStream {
        let value = match &self.kind {
            FieldKind::Primitive {
                default: Some(default),
            } => {
                let default = default.get();
                quote!(#default)
            }

            FieldKind::Primitive { default: None } => quote!(Default::default()),

            FieldKind::State => {
                let ty = self.ty;
                let tag = *self.tag.get();
                quote!(<#ty>::new(path.child(#tag)))
            }
        };

        get_init(&self.name, self.index, value)
    }

    pub fn to_accessors(&self, path: &PathField<'_>) -> proc_macro2::TokenStream {
        let getter = self.to_getter();
        let mut_getter = self.to_mut_getter();
        let setter = self.to_setter(path);

        quote! {
            #getter
            #mut_getter
            #setter
        }
    }

    fn to_getter(&self) -> proc_macro2::TokenStream {
        let doc = format!("Gets {}.", get_desc(&self.name, self.index));
        let name = get_name(&self.name, self.index);
        let ty = self.ty;
        let access = get_access(&self.name, self.index);

        quote! {
            #[doc = #doc]
            #[inline]
            pub fn #name(&self) -> &#ty {
                &self.#access
            }
        }
    }

    fn to_mut_getter(&self) -> proc_macro2::TokenStream {
        let doc = format!("Gets mutable {}.", get_desc(&self.name, self.index));
        let name = format_ident!("{}_mut", get_name(&self.name, self.index));
        let ty = self.ty;
        let access = get_access(&self.name, self.index);

        quote! {
            #[doc = #doc]
            #[inline]
            pub fn #name(&mut self) -> &mut #ty {
                &mut self.#access
            }
        }
    }

    fn to_setter(&self, path: &PathField<'_>) -> proc_macro2::TokenStream {
        let doc = format!("Sets {}.", get_desc(&self.name, self.index));
        let ty = self.ty;
        let access = get_access(&self.name, self.index);
        let tag = *self.tag.get();

        match self.kind {
            FieldKind::Primitive { .. } => {
                let name = format_ident!("set_{}", get_name(&self.name, self.index));

                quote! {
                    #[doc = #doc]
                    pub fn #name(&mut self, value: #ty) -> &mut Self {
                        // TODO: Track changes
                        self.#access = value;
                        self
                    }
                }
            }

            FieldKind::State => {
                let name = format_ident!("set_{}_with", get_name(&self.name, self.index));
                let path = get_access(&path.name, path.index);

                quote! {
                    #[doc = #doc]
                    pub fn #name<F: FnOnce(Path) -> #ty>(&mut self, get_value: F) -> &mut Self {
                        // TODO: Track changes
                        self.#access = get_value(self.#path.child(#tag));
                        self
                    }
                }
            }
        }
    }

    pub fn to_sizer(&self) -> proc_macro2::TokenStream {
        let tag = *self.tag.get() as u32;
        let access = get_access(&self.name, self.index);

        let (sizer, wire_type) = match self.kind {
            FieldKind::Primitive { .. } => (quote!(), 0u32),
            FieldKind::State => (quote!(size += self.#access.size().size();), 2),
        };

        quote! {
            size += (#tag << 3 | #wire_type).size();
            #sizer
            size += self.#access.size();
        }
    }

    pub fn to_serializer(&self) -> proc_macro2::TokenStream {
        let tag = *self.tag.get() as u32;
        let access = get_access(&self.name, self.index);

        let (sizer, wire_type) = match self.kind {
            FieldKind::Primitive { .. } => (quote!(), 0u32),
            FieldKind::State => (quote!(self.#access.size().serialize(writer)?;), 2),
        };

        quote! {
            (#tag << 3 | #wire_type).serialize(writer)?;
            #sizer
            self.#access.serialize(writer)?;
        }
    }

    pub fn to_deserializer(&self) -> proc_macro2::TokenStream {
        let tag = *self.tag.get();
        let access = get_access(&self.name, self.index);

        let (deserializer, wire_type) = match self.kind {
            FieldKind::Primitive { .. } => (quote!(self.#access.deserialize(reader)?;), 0u8),
            FieldKind::State => (
                quote! {
                    let size = varint::Varint::deserialize(reader)?;
                    self.#access.deserialize(&mut reader.by_ref().take(size))?;
                },
                2,
            ),
        };

        quote!(#tag if wire_type == #wire_type => {
            #deserializer
        })
    }
}

pub struct PathField<'a> {
    name: Option<syn::Ident>,
    ty: &'a syn::Type,
    index: usize,
}

impl<'a> PathField<'a> {
    pub fn new(field: &'a syn::Field, index: usize) -> Self {
        Self {
            name: field.ident.clone(),
            ty: &field.ty,
            index,
        }
    }

    pub fn to_arg(&self) -> proc_macro2::TokenStream {
        let ty = self.ty;
        quote!(path: #ty)
    }

    pub fn to_init(&self) -> proc_macro2::TokenStream {
        get_init(&self.name, self.index, quote!(path))
    }
}

fn get_access(name: &Option<syn::Ident>, index: usize) -> proc_macro2::TokenStream {
    use quote::ToTokens;

    match name {
        Some(name) => quote!(#name),
        None => syn::Index::from(index).into_token_stream(),
    }
}

fn get_init(
    name: &Option<syn::Ident>,
    index: usize,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let access = get_access(name, index);
    quote!(#access: #value)
}

fn get_name(name: &Option<syn::Ident>, index: usize) -> syn::Ident {
    match name {
        Some(name) => name.to_owned(),
        None => format_ident!("f_{}", index),
    }
}

fn get_desc(name: &Option<syn::Ident>, index: usize) -> String {
    match name {
        Some(name) => format!("`{}`", name),
        None => format!("field #{}", index),
    }
}
