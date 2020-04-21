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
        let variants = parse_variants(context, impler, setting, &attrs, variants)?;

        Ok(Self {
            impler,
            setting,
            variants,
        })
    }

    fn impl_ctors(&self) -> TokenStream {
        let ctors = self.variants.iter().map(|r#struct| r#struct.ctor());

        self.impler.impl_with(
            if self.setting.state {
                &["State"]
            } else {
                &["Default"]
            },
            quote!(#(#ctors)*),
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

        self.impler.impl_for_with(
            "SerializeV2",
            &["SerializeNested"],
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
            let ctor_name = variant.ctor_name();
            let qual = variant.qual();
            let tag = variant.tag();

            let args = if self.setting.state {
                quote!(self.runtime().parent())
            } else {
                quote!()
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
            if self.setting.state {
                &["MergeNested", "State"]
            } else {
                &["Default", "MergeNested"]
            },
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
}

fn parse_variants<'a>(
    context: &'a Context,
    impler: &'a Impler,
    setting: &'a DeriveSetting,
    attrs: &EnumAttrs,
    variants: &mut syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> derive::Result<Vec<Struct<'a>>> {
    let mut parsed = Vec::with_capacity(variants.iter().len());

    let reserved: HashSet<_> = attrs.reserved.iter().collect();
    let mut tags = HashSet::new();
    let mut unique = true;

    for variant in variants.iter_mut() {
        if variant.discriminant.is_some() {
            context.error(variant, "discriminant is not supported yet");
            continue;
        }

        if let Ok((parsed_variant, unknown_attrs)) = Variant::parse(context, variant) {
            let (tag, tag_tokens) = parsed_variant.tag_with_tokens();

            if reserved.contains(&tag) {
                context.error(tag_tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                context.error(tag_tokens, format!("duplicate tag {}", tag));
                unique = false;
            }

            if let Ok(r#struct) = Struct::parse(
                context,
                impler,
                setting,
                unknown_attrs,
                &mut variant.fields,
                Some(parsed_variant),
            ) {
                parsed.push(r#struct);
            }
        }
    }

    if parsed.len() == parsed.capacity() && unique {
        Ok(parsed)
    } else {
        Err(())
    }
}

impl<'a> ToTokens for Enum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.setting.has_ctors() {
            tokens.extend(self.impl_ctors());
        }

        tokens.extend(self.impl_wire_type());

        if self.setting.serialize {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.merge {
            tokens.extend(self.impl_merge());
        }
    }
}
