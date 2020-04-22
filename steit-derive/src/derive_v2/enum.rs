use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{AttrParse, VecAttr},
    context::Context,
    impler::Impler,
};

use super::{
    derive::{self, DeriveSetting},
    r#struct::Struct,
    variant::Variant,
};

struct EnumAttrs {
    reserved: Vec<u32>,
}

impl EnumAttrs {
    pub fn parse(context: &Context, attrs: impl AttrParse) -> Self {
        let mut reserved = VecAttr::new(context, "reserved");

        attrs.parse(context, true, |meta| match meta {
            syn::Meta::List(meta) if reserved.parse_int_list(meta) => true,
            _ => false,
        });

        Self {
            reserved: reserved.get(),
        }
    }
}

pub struct Enum<'a> {
    impler: &'a Impler<'a>,
    setting: &'a DeriveSetting,
    variants: Vec<Struct<'a>>,
    default_variant_index: Option<usize>,
}

impl<'a> Enum<'a> {
    pub fn parse(
        context: &'a Context,
        impler: &'a Impler,
        setting: &'a DeriveSetting,
        attrs: impl AttrParse,
        variants: &mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> derive::Result<Self> {
        if variants.is_empty() {
            context.error(variants, "cannot derive for enums with zero variants");
            return Err(());
        }

        let attrs = EnumAttrs::parse(context, attrs);

        let (variants, default_variant_index) =
            parse_variants(context, impler, setting, &attrs, variants)?;

        Ok(Self {
            impler,
            setting,
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

    fn impl_wire_type(&self) -> TokenStream {
        self.impler.impl_for_with(
            "HasWireType",
            &[],
            quote! {
                const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
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
                let destructure = size_cache.destructure(quote!(size_cache));
                quote!(#name #qual { #destructure, .. } => Some(size_cache))
            } else {
                quote!(#name #qual { .. } => None)
            }
        });

        self.impler.impl_for(
            "SerializeV2",
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

    fn impl_merge(&self) -> TokenStream {
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
            "MergeV2",
            self.trait_bounds(&["Default", "MergeV2"]),
            quote! {
                fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                    let tag = u32::deserialize_v2(reader)?;

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
            let destructure = runtime.destructure(quote!(runtime));

            quote!(#name #qual { #destructure, .. } => runtime)
        });

        let runtime_setters = self.variants.iter().map(|r#struct| {
            let variant = r#struct.variant().unwrap();
            let qual = variant.qual();
            let tag = variant.tag();

            let destructure = r#struct.destructure();

            let runtime = r#struct.runtime().unwrap();
            let runtime_destructure = runtime.destructure(quote!(self_runtime));

            let runtime_setter = r#struct.runtime_setter();

            quote! {
                #name #qual { #destructure #runtime_destructure, .. } => {
                    let runtime = runtime.nested(#tag as u16);
                    #runtime_setter
                }
            }
        });

        self.impler.impl_for(
            "StateV2",
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
            },
        )
    }
}

fn parse_variants<'a>(
    context: &'a Context,
    impler: &'a Impler,
    setting: &'a DeriveSetting,
    attrs: &EnumAttrs,
    variants: &mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> derive::Result<(Vec<Struct<'a>>, Option<usize>)> {
    let mut parsed_variants = Vec::with_capacity(variants.iter().len());

    let reserved_tags: HashSet<_> = attrs.reserved.iter().collect();
    let mut tags = HashSet::new();
    let mut unique_tags = true;

    let mut default_variant_index = None;

    for variant in variants.iter_mut() {
        if variant.discriminant.is_some() {
            context.error(variant, "discriminant is not supported yet");
            continue;
        }

        if let Ok((parsed_variant, unknown_attrs)) = Variant::parse(context, variant) {
            let (tag, tag_tokens) = parsed_variant.tag_with_tokens();
            let (default, default_tokens) = parsed_variant.default_with_tokens();

            if reserved_tags.contains(&tag) {
                context.error(tag_tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                context.error(tag_tokens, format!("duplicate tag {}", tag));
                unique_tags = false;
            }

            if default {
                if default_variant_index.is_none() {
                    // If struct parsing fails, `default_variant_index` could be wrong.
                    // We accept that, so we don't miss any multiple-default-variants error.
                    default_variant_index = Some(parsed_variants.len());
                } else {
                    context.error(
                        default_tokens.unwrap(),
                        "unexpected multiple default variants",
                    );
                }
            }

            if let Ok(r#struct) = Struct::parse(
                context,
                impler,
                setting,
                unknown_attrs,
                &mut variant.fields,
                Some(parsed_variant),
            ) {
                parsed_variants.push(r#struct);
            }
        }
    }

    if default_variant_index.is_none() {
        context.error(
            impler.name(),
            "expected a default variant #[steit(tag = …, default)]",
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

        if self.setting.impl_default() {
            tokens.extend(self.impl_default());
        }

        tokens.extend(self.impl_wire_type());

        if self.setting.impl_serialize() {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.impl_merge() {
            tokens.extend(self.impl_merge());
        }

        if self.setting.impl_state() {
            tokens.extend(self.impl_state());
        }
    }
}
