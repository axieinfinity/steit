use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{AttrParse, VecAttr},
    ctx::Context,
    r#impl::Impl,
};

use super::{
    derive::{self, DeriveSetting},
    r#struct::Struct,
    variant::Variant,
};

struct EnumAttrs {
    reserved: Vec<u16>,
}

impl EnumAttrs {
    pub fn parse(context: &Context, attrs: impl AttrParse) -> Self {
        let mut reserved = VecAttr::new(context, "reserved");

        attrs.parse(context, true, &mut |meta| match meta {
            syn::Meta::List(meta) if reserved.parse_int_list(meta) => true,
            _ => false,
        });

        Self {
            reserved: reserved.get(),
        }
    }
}

pub struct Enum<'a> {
    setting: &'a DeriveSetting,
    context: &'a Context,
    r#impl: &'a Impl<'a>,
    variants: Vec<Struct<'a>>,
}

impl<'a> Enum<'a> {
    pub fn parse(
        setting: &'a DeriveSetting,
        context: &'a Context,
        r#impl: &'a Impl,
        attrs: impl AttrParse,
        data: &'a mut syn::DataEnum,
    ) -> derive::Result<Self> {
        if data.variants.is_empty() {
            context.error(&data.variants, "cannot derive for enums with zero variants");
            return Err(());
        }

        Self::parse_variants(setting, context, r#impl, attrs, &mut data.variants).map(|variants| {
            Self {
                setting,
                context,
                r#impl,
                variants,
            }
        })
    }

    fn parse_variants(
        setting: &'a DeriveSetting,
        context: &'a Context,
        r#impl: &'a Impl,
        attrs: impl AttrParse,
        variants: &'a mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> derive::Result<Vec<Struct<'a>>> {
        let len = variants.iter().len();
        let mut parsed_data = Vec::with_capacity(len);

        for variant in variants.iter() {
            if variant.discriminant.is_some() {
                context.error(&variants, "cannot derive for enums with discriminants");
                return Err(());
            }
        }

        let attrs = EnumAttrs::parse(context, attrs);

        for variant in variants.iter_mut() {
            if let Ok((parsed, unknown_attrs)) = Variant::parse(context, variant) {
                parsed_data.push((parsed, unknown_attrs, &mut variant.fields));
            }
        }

        if parsed_data.len() != len {
            return Err(());
        }

        let reserved: HashSet<_> = attrs.reserved.iter().collect();
        let mut tags = HashSet::new();
        let mut unique = true;
        let mut named_hint = None;

        for (variant, _, fields) in &parsed_data {
            let (tag, tokens) = variant.tag_with_tokens();

            if reserved.contains(&tag) {
                context.error(tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                context.error(tokens, "duplicate tag");
                unique = false;
            }

            match fields {
                syn::Fields::Named(_) => match named_hint {
                    Some(false) => unreachable!("enum is named and unnamed at the same time"),
                    _ => named_hint = Some(true),
                },

                syn::Fields::Unnamed(_) => match named_hint {
                    Some(true) => unreachable!("enum is named and unnamed at the same time"),
                    _ => named_hint = Some(false),
                },

                syn::Fields::Unit => (),
            }
        }

        let mut parsed = Vec::with_capacity(len);

        for (variant, unknown_attrs, fields) in parsed_data {
            if let Ok(r#struct) = Struct::parse(
                setting,
                context,
                r#impl,
                unknown_attrs,
                fields,
                named_hint,
                variant,
            ) {
                parsed.push(r#struct);
            }
        }

        if parsed.len() != len {
            return Err(());
        }

        if unique {
            Ok(parsed)
        } else {
            Err(())
        }
    }

    fn state_bounds(&self) -> &[&str] {
        if self.setting.runtime() {
            &["State"]
        } else {
            &[]
        }
    }

    fn impl_ctors(&self) -> TokenStream {
        let default_ctor_name = self.variants.iter().find_map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            if variant.tag() == 0 {
                Some(r#struct.ctor_name())
            } else {
                None
            }
        });

        let ctors = self.variants.iter().map(|r#struct| r#struct.ctor());

        self.r#impl.r#impl_with(
            self.state_bounds(),
            if let Some(default_ctor_name) = default_ctor_name {
                let (declare_arg, call_arg) = match self.setting.runtime() {
                    true => (quote!(runtime: Runtime), quote!(runtime)),
                    false => (quote!(), quote!()),
                };

                quote! {
                    #[inline]
                    pub fn new(#declare_arg) -> Self {
                        Self::#default_ctor_name(#call_arg)
                    }

                    #(#ctors)*
                }
            } else {
                if self.setting.default() {
                    self.context.error(
                        self.r#impl.name(),
                        "expected a variant with tag 0 as the default variant of this enum",
                    );
                }

                quote!(#(#ctors)*)
            },
        )
    }

    fn impl_cached_size(&self) -> TokenStream {
        let name = self.r#impl.name();

        let cached_sizes = self.variants.iter().map(|r#struct| {
            let cached_size = r#struct
                .cached_size()
                .unwrap_or_else(|| unreachable!("expected a `Runtime` field"));

            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let qual = variant.qual();
            let destructure = cached_size.destructure(quote!(cached_size));

            quote!(#name #qual { #destructure, .. } => cached_size)
        });

        self.r#impl.r#impl(quote! {
            #[inline]
            fn cached_size(&self) -> &CachedSize {
                match self { #(#cached_sizes,)* }
            }
        })
    }

    fn impl_setters(&self) -> TokenStream {
        let setters = self.variants.iter().map(|r#struct| r#struct.setters());

        self.r#impl
            .r#impl_with(self.state_bounds(), quote!(#(#setters)*))
    }

    fn impl_default(&self) -> TokenStream {
        let arg = match self.setting.runtime() {
            true => quote!(Runtime::default()),
            false => quote!(),
        };

        self.r#impl.impl_for_with(
            "Default",
            self.state_bounds(),
            quote! {
                #[inline]
                fn default() -> Self {
                    Self::new(#arg)
                }
            },
        )
    }

    fn impl_wire_type(&self) -> TokenStream {
        self.r#impl.impl_for_with(
            "WireType",
            &[],
            quote! {
                const WIRE_TYPE: u8 = 2;
            },
        )
    }

    fn impl_serialize(&self) -> TokenStream {
        let name = self.r#impl.name();

        let sizers = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let tag = variant.tag();
            let qual = variant.qual();

            let destructure = r#struct.destructure();
            let sizer = r#struct.sizer();

            quote! {
                #name #qual { #destructure .. } => {
                    size += #tag.compute_size();
                    #sizer
                }
            }
        });

        let (set_cached_size, cached_size) = if self.setting.cached_size() {
            (
                quote! { self.cached_size().set(size); },
                quote! {
                    #[inline]
                    fn cached_size(&self) -> u32 {
                        self.cached_size().get()
                    }
                },
            )
        } else {
            (quote!(), quote!())
        };

        let serializers = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let tag = variant.tag();
            let qual = variant.qual();

            let destructure = r#struct.destructure();
            let serializer = r#struct.serializer();

            quote! {
                #name #qual { #destructure .. } => {
                    #tag.serialize(writer)?;
                    #serializer
                }
            }
        });

        self.r#impl.impl_for(
            "Serialize",
            quote! {
                fn compute_size(&self) -> u32 {
                    let mut size = 0;
                    match self { #(#sizers)* }
                    #set_cached_size
                    size
                }

                fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    match self { #(#serializers)* }
                    Ok(())
                }

                #cached_size
            },
        )
    }

    fn impl_merge(&self) -> TokenStream {
        let name = self.r#impl.name();

        let mergers = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let tag = variant.tag();
            let qual = variant.qual();
            let ctor_name = variant.ctor_name();

            let arg = match self.setting.runtime() {
                true => quote!(self.runtime().parent()),
                false => quote!(),
            };

            let destructure = r#struct.destructure();
            let merger = r#struct.merger();

            quote! {
                #tag => {
                    if let #name #qual { .. } = self {
                    } else {
                        *self = Self::#ctor_name(#arg);
                    }

                    if let #name #qual { #destructure .. } = self {
                        #merger
                    }
                }
            }
        });

        self.r#impl.impl_for_with(
            "Merge",
            self.state_bounds(),
            quote! {
                fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
                    let tag = u16::deserialize(reader)?;

                    match tag {
                        #(#mergers)*

                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("unexpected variant tag {}", tag),
                            ));
                        }
                    }

                    Ok(())
                }
            },
        )
    }

    fn impl_state(&self) -> TokenStream {
        let name = self.r#impl.name();

        let runtimes = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let runtime = r#struct
                .runtime()
                .unwrap_or_else(|| unreachable!("expected a `Runtime` field"));

            let qual = variant.qual();
            let destructure = runtime.destructure(quote!(runtime));

            quote!(#name #qual { #destructure, .. } => runtime)
        });

        let runtime_setters = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let runtime = r#struct
                .runtime()
                .unwrap_or_else(|| unreachable!("expected a `Runtime` field"));

            let tag = variant.tag();
            let qual = variant.qual();

            let destructure = r#struct.destructure();
            let runtime_destructure = runtime.destructure(quote!(current_runtime));

            let runtime_setter = r#struct.runtime_setter();

            quote! {
                #name #qual { #destructure #runtime_destructure, .. } => {
                    let runtime = runtime.nested(#tag);
                    #runtime_setter
                }
            }
        });

        let replayers = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let tag = variant.tag();
            let qual = variant.qual();

            let destructure = r#struct.destructure();
            let replayer = r#struct.replayer();

            quote! {
                #tag => {
                    if let #name #qual { #destructure .. } = self {
                        #replayer
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("expected variant with tag {}, got another", tag),
                        ))
                    }
                }
            }
        });

        self.r#impl.impl_for(
            "State",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::new(runtime)
                }

                #[inline]
                fn runtime(&self) -> &Runtime {
                    match self { #(#runtimes,)* }
                }

                #[inline]
                fn set_runtime(&mut self, runtime: Runtime) {
                    match self { #(#runtime_setters)* }
                }

                #[inline]
                fn handle<'a>(
                    &mut self,
                    path: &mut impl Iterator<Item = &'a u16>,
                    kind: &ReplayKind,
                    reader: &mut Eof<impl io::Read>,
                ) -> io::Result<()> {
                    if let Some(tag) = path.next() {
                        match tag {
                            #(#replayers,)*

                            _ => Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("unexpected variant tag {}", tag),
                            )),
                        }
                    } else {
                        match kind {
                            ReplayKind::Update => self.handle_update(reader),

                            ReplayKind::Add | ReplayKind::Remove => Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "`add` and `remove` are not supported on enums",
                            )),
                        }
                    }
                }

                #[inline]
                fn is_root(&self) -> bool {
                    self.runtime().parent().is_root()
                }
            },
        )
    }

    fn impl_meta(&self) -> TokenStream {
        let name = self.r#impl.name().to_token_stream().to_string();

        let variants = self.variants.iter().map(|r#struct| {
            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let meta = r#struct.meta();
            let tag = variant.tag();

            quote! {
                Variant {
                    ty: #meta,
                    tag: #tag,
                }
            }
        });

        self.r#impl.impl_for_with(
            "HasMeta",
            &["IsFieldType"],
            quote! {
                const META: &'static Meta = &Meta::Enum(&Enum {
                    name: #name,
                    variants: &[#(#variants,)*],
                });

                const META_NAME: &'static str = #name;
            },
        )
    }

    fn impl_field_type(&self) -> TokenStream {
        self.r#impl.impl_for(
            "IsFieldType",
            quote! {
                const FIELD_TYPE: &'static FieldType = &FieldType::Meta(Self::META);
                const FIELD_TYPE_REF: &'static FieldType = &FieldType::MetaRef(Self::META_NAME);
            },
        )
    }
}

impl<'a> ToTokens for Enum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.setting.ctors(self.context, true) {
            tokens.extend(self.impl_ctors());
        }

        if self.setting.setters(self.context) {
            tokens.extend(self.impl_setters());
        }

        if self.setting.cached_size() {
            tokens.extend(self.impl_cached_size());
        }

        if self.setting.default() {
            tokens.extend(self.impl_default());
        }

        tokens.extend(self.impl_wire_type());

        if self.setting.serialize {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.merge {
            tokens.extend(self.impl_merge());
        }

        if self.setting.state {
            tokens.extend(self.impl_state());
        }

        if self.setting.meta() {
            tokens.extend(self.impl_meta());
            tokens.extend(self.impl_field_type());
        }
    }
}
