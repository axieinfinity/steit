use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attribute, AttributeParse, VecAttribute},
    ctx::Context,
    r#impl::Implementer,
};

use super::{
    derive::{self, DeriveSetting},
    r#struct::Struct,
    variant::Variant,
};

struct EnumAttrs {
    reserved_tags: Vec<u32>,
    csharp_name: Option<String>,
}

impl EnumAttrs {
    pub fn parse(ctx: &Context, attrs: impl AttributeParse) -> Self {
        let mut reserved_tags = VecAttribute::new(ctx, "reserved_tags");
        let mut csharp_name = Attribute::new(ctx, "csharp_name");

        attrs.parse(ctx, true, |meta| match meta {
            syn::Meta::List(meta) if reserved_tags.parse_int_list(meta) => true,
            syn::Meta::NameValue(meta) if csharp_name.parse_str(meta) => true,
            _ => false,
        });

        Self {
            reserved_tags: reserved_tags.get(),
            csharp_name: csharp_name.get(),
        }
    }
}

pub struct Enum<'a> {
    impler: &'a Implementer<'a>,
    setting: &'a DeriveSetting,
    attrs: EnumAttrs,
    type_params: &'a [&'a syn::TypeParam],
    variants: Vec<Struct<'a>>,
    default_variant_index: Option<usize>,
}

impl<'a> Enum<'a> {
    pub fn parse(
        ctx: &'a Context,
        impler: &'a Implementer,
        setting: &'a DeriveSetting,
        attrs: impl AttributeParse,
        type_params: &'a [&'a syn::TypeParam],
        variants: &mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> derive::Result<Self> {
        if variants.is_empty() {
            ctx.error(variants, "cannot derive for enums with zero variants");
            return Err(());
        }

        let attrs = EnumAttrs::parse(ctx, attrs);

        let (variants, default_variant_index) =
            parse_variants(ctx, impler, setting, &attrs, type_params, variants)?;

        Ok(Self {
            impler,
            setting,
            attrs,
            type_params,
            variants,
            default_variant_index,
        })
    }

    pub fn default_variant(&self) -> Option<&Struct<'a>> {
        self.default_variant_index
            .and_then(|index| self.variants.get(index))
    }

    fn trait_bounds(&self, fallback: &'static [&str]) -> &[&str] {
        if self.setting.impl_state() {
            &["State"]
        } else {
            fallback
        }
    }

    fn impl_ctors(&self) -> TokenStream {
        let ctors = self.variants.iter().map(|r#struct| r#struct.ctor());

        let (default_ctor_params, default_ctor_args) = if self.setting.impl_state() {
            (Some(quote!(runtime: Runtime)), Some(quote!(runtime)))
        } else {
            Default::default()
        };

        let default_ctor = if let Some(default_variant) = self.default_variant() {
            let default_ctor_name = default_variant.ctor_name();
            quote!(Self::#default_ctor_name(#default_ctor_args))
        } else {
            quote!(unimplemented!("no default variant"))
        };

        self.impler.impl_with(
            self.trait_bounds(&["Default"]),
            quote! {
                #[inline]
                pub fn empty(#default_ctor_params) -> Self {
                    #default_ctor
                }

                #(#ctors)*
            },
        )
    }

    fn impl_setters(&self) -> TokenStream {
        let setters = self.variants.iter().map(|r#struct| r#struct.setters());

        self.impler
            .impl_with(self.trait_bounds(&["Default"]), quote!(#(#setters)*))
    }

    fn impl_eq(&self) -> TokenStream {
        let name = self.impler.name();

        let eqs = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();

            let destructure = r#struct.destructure();
            let other_destructure = r#struct.destructure_prefixed(format_ident!("other"));
            let eq = r#struct.eq();

            quote! {
                #name #qual { #destructure .. } => {
                    if let #name #qual { #other_destructure .. } = other {
                        #eq
                    } else {
                        false
                    }
                }
            }
        });

        let mut r#impl = self.impler.impl_for(
            "PartialEq",
            quote! {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    match self { #(#eqs,)* }
                }
            },
        );

        r#impl.extend(self.impler.impl_for("Eq", quote!()));
        r#impl
    }

    fn impl_default(&self) -> TokenStream {
        let args = if self.setting.impl_state() {
            Some(quote!(Runtime::default()))
        } else {
            None
        };

        self.impler.impl_for_with(
            "Default",
            self.trait_bounds(&["Default"]),
            quote! {
                #[inline]
                fn default() -> Self {
                    Self::empty(#args)
                }
            },
        )
    }

    fn impl_hash(&self) -> TokenStream {
        let name = self.impler.name();

        let hashers = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

            let destructure = r#struct.destructure();
            let hasher = r#struct.hasher();

            quote! {
                #name #qual { #destructure .. } => {
                    #tag.hash(state);
                    #hasher
                }
            }
        });

        self.impler.impl_for(
            "Hash",
            quote! {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    match self { #(#hashers)* }
                }
            },
        )
    }

    fn impl_wire_type(&self) -> TokenStream {
        self.impler.impl_for_with(
            "HasWireType",
            &[],
            quote! {
                const WIRE_TYPE: WireType = WireType::Sized;
            },
        )
    }

    fn impl_serialize(&self) -> TokenStream {
        let name = self.impler.name();

        let sizers = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

            let destructure = r#struct.destructure();
            let sizer = r#struct.sizer();

            quote! {
                #name #qual { #destructure .. } => {
                    size += #tag.cache_size();
                    #sizer
                }
            }
        });

        let serializers = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

            let destructure = r#struct.destructure();
            let serializer = r#struct.serializer();

            quote! {
                #name #qual { #destructure .. } => {
                    #tag.serialize_cached(writer)?;
                    #serializer
                }
            }
        });

        let size_caches = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();

            if let Some(size_cache) = r#struct.size_cache() {
                let destructure = size_cache.destructure(format_ident!("size_cache"));
                quote!(#name #qual { #destructure, .. } => Some(size_cache))
            } else {
                quote!(#name #qual { .. } => None)
            }
        });

        self.impler.impl_for(
            "Serialize",
            quote! {
                fn compute_size(&self) -> u32 {
                    let mut size = 0;
                    match self { #(#sizers)* }
                    size
                }

                fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    match self { #(#serializers)* }
                    Ok(())
                }

                fn size_cache(&self) -> Option<&SizeCache> {
                    match self { #(#size_caches,)* }
                }
            },
        )
    }

    fn impl_deserialize(&self) -> TokenStream {
        let name = self.impler.name();

        let mergers = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();
            let ctor_name = variant.ctor_name();

            let args = if self.setting.impl_state() {
                Some(quote!(self.runtime().parent()))
            } else {
                None
            };

            let destructure = r#struct.destructure();
            let merger = r#struct.merger();

            quote! {
                #tag => {
                    if let #name #qual { .. } = self {
                    } else {
                        *self = Self::#ctor_name(#args);
                    }

                    if let #name #qual { #destructure .. } = self {
                        #merger
                    }
                }
            }
        });

        self.impler.impl_for_with(
            "Deserialize",
            self.trait_bounds(&["Deserialize"]),
            quote! {
                fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                    let tag = u32::deserialize(reader)?;

                    match tag {
                        #(#mergers)*

                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("unknown variant tag {}", tag),
                            ));
                        }
                    }

                    Ok(())
                }
            },
        )
    }

    fn impl_state(&self) -> TokenStream {
        let name = self.impler.name();

        let runtimes = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();

            let runtime = r#struct.runtime().unwrap();
            let destructure = runtime.destructure(format_ident!("runtime"));

            quote!(#name #qual { #destructure, .. } => runtime)
        });

        let runtime_setters = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

            let destructure = r#struct.destructure();

            let runtime = r#struct.runtime().unwrap();
            let runtime_destructure = runtime.destructure(format_ident!("self_runtime"));

            let runtime_setter = r#struct.runtime_setter();

            quote! {
                #name #qual { #destructure #runtime_destructure, .. } => {
                    let runtime = runtime.nested(#tag);
                    #runtime_setter
                }
            }
        });

        let replayers = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

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

        self.impler.impl_for(
            "State",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::empty(runtime)
                }

                fn runtime(&self) -> &Runtime {
                    match self { #(#runtimes,)* }
                }

                fn set_runtime(&mut self, runtime: Runtime) {
                    match self { #(#runtime_setters)* }
                }

                #[inline]
                fn is_root(&self) -> bool {
                    self.runtime().parent().is_root()
                }

                #[inline]
                fn handle_update(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                    *self = Self::with_runtime(self.runtime().parent());
                    self.merge(reader)
                }

                fn handle(
                    &mut self,
                    mut path: impl Iterator<Item = u32>,
                    kind: LogEntryKind,
                    key: Option<u32>,
                    reader: &mut Reader<impl io::Read>,
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
                            LogEntryKind::Update => self.handle_update(reader),

                            _ => Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("{:?} is not supported on `{}`", kind, stringify!(#name)),
                            )),
                        }
                    }
                }
            },
        )
    }

    fn impl_meta(&self) -> TokenStream {
        let rust_name = self.impler.name().to_string();

        let csharp_name = match &self.attrs.csharp_name {
            Some(csharp_name) => quote!(Some(#csharp_name)),
            None => quote!(None),
        };

        let builtin = self.setting.steit_owned;

        let variants = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let tag = variant.tag();

            let meta = r#struct.meta();

            quote! {
                VariantMeta {
                    ty: #meta,
                    tag: #tag,
                }
            }
        });

        let param_meta_list = self.type_params.iter().map(|param| {
            let name = &param.ident;
            quote!(FieldTypeMeta::Type(#name::TYPE))
        });

        let type_params = self.type_params.iter().map(|type_param| {
            let type_param = type_param.ident.to_string();
            quote!(#type_param)
        });

        let links = self.variants.iter().map(|r#struct| r#struct.meta_links());

        self.impler.impl_for(
            "HasMeta",
            quote! {
                const NAME: &'static NameMeta = &NameMeta {
                    rust: #rust_name,
                    csharp: #csharp_name,
                };

                const TYPE: &'static TypeMeta = &TypeMeta::Ref(Self::NAME, &[#(#param_meta_list,)*]);

                const LINK: &'static MetaLink = &MetaLink {
                    r#type: Self::TYPE,
                    msg: Some(MessageMeta::Enum(EnumMeta {
                        name: Self::NAME,
                        type_params: &[#(#type_params,)*],
                        variants: &[#(#variants,)*],
                        builtin: #builtin,
                    })),
                    links: || &[#(#links)*],
                };
            },
        )
    }
}

fn parse_variants<'a>(
    ctx: &'a Context,
    impler: &'a Implementer,
    setting: &'a DeriveSetting,
    attrs: &EnumAttrs,
    type_params: &'a [&'a syn::TypeParam],
    variants: &mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> derive::Result<(Vec<Struct<'a>>, Option<usize>)> {
    let mut parsed_variants = Vec::with_capacity(variants.iter().len());

    let reserved_tags: HashSet<_> = attrs.reserved_tags.iter().collect();
    let mut tags = HashSet::new();
    let mut unique_tags = true;

    let mut default_variant_index = None;

    for variant in variants.iter_mut() {
        if variant.discriminant.is_some() {
            ctx.error(variant, "discriminants are not supported yet");
            continue;
        }

        if let Ok((parsed_variant, unknown_attrs)) = Variant::parse(ctx, variant) {
            let (tag, tag_tokens) = parsed_variant.tag_with_tokens();

            if reserved_tags.contains(&tag) {
                ctx.error(tag_tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                ctx.error(tag_tokens, format!("duplicate tag {}", tag));
                unique_tags = false;
            }

            if let Ok(r#struct) = Struct::parse(
                ctx,
                impler,
                setting,
                unknown_attrs,
                type_params,
                &mut variant.fields,
                Some(parsed_variant),
            ) {
                if tag == 0 && default_variant_index.is_none() {
                    default_variant_index = Some(parsed_variants.len());
                }

                parsed_variants.push(r#struct);
            }
        }
    }

    if default_variant_index.is_none() {
        ctx.error(
            impler.name(),
            "expected a variant with tag 0 as the default variant `#[steit(tag = 0)]`",
        );
    }

    if parsed_variants.len() == parsed_variants.capacity() && unique_tags {
        Ok((parsed_variants, default_variant_index))
    } else {
        Err(())
    }
}

impl<'a> ToTokens for Enum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.impl_ctors());
        tokens.extend(self.impl_setters());

        if self.setting.has_eq() {
            tokens.extend(self.impl_eq());
        }

        tokens.extend(self.impl_default());

        if self.setting.has_hash() {
            tokens.extend(self.impl_hash());
        }

        tokens.extend(self.impl_wire_type());

        if self.setting.impl_serialize() {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.impl_deserialize() {
            tokens.extend(self.impl_deserialize());
        }

        if self.setting.impl_state() {
            tokens.extend(self.impl_state());
        }

        if self.setting.has_meta() {
            tokens.extend(self.impl_meta());
        }
    }
}
