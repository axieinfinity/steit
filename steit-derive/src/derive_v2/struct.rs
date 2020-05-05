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
    field::{DeriveField, Field},
    variant::Variant,
};

struct StructAttrs {
    no_size_cache: bool,

    size_cache_renamed: Option<(String, TokenStream)>,
    runtime_renamed: Option<(String, TokenStream)>,

    reserved: Vec<u32>,
}

impl StructAttrs {
    pub fn parse(ctx: &Context, attrs: impl AttributeParse) -> Self {
        let mut no_size_cache = Attribute::new(ctx, "no_size_cache");

        let mut size_cache_renamed = Attribute::new(ctx, "size_cache_renamed");
        let mut runtime_renamed = Attribute::new(ctx, "runtime_renamed");

        let mut reserved = VecAttribute::new(ctx, "reserved");

        attrs.parse(ctx, true, |meta| match meta {
            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if size_cache_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            syn::Meta::List(meta) if reserved.parse_int_list(meta) => true,

            _ => false,
        });

        Self {
            no_size_cache: no_size_cache.get().unwrap_or_default(),

            size_cache_renamed: size_cache_renamed.get_with_tokens(),
            runtime_renamed: runtime_renamed.get_with_tokens(),

            reserved: reserved.get(),
        }
    }
}

pub struct Struct<'a> {
    impler: &'a Implementer<'a>,
    setting: &'a DeriveSetting,
    type_params: &'a [&'a syn::TypeParam],
    fields: Vec<DeriveField<'a>>,
    size_cache: Option<Field>,
    runtime: Option<Field>,
    variant: Option<Variant>,
}

macro_rules! map_fields {
    ($struct:ident, _.$($tail:tt)*) => {
        $struct.fields.iter().map(|field| field.$($tail)*)
    };
}

impl<'a> Struct<'a> {
    pub fn parse(
        ctx: &'a Context,
        impler: &'a Implementer,
        setting: &'a DeriveSetting,
        attrs: impl AttributeParse,
        type_params: &'a [&'a syn::TypeParam],
        fields: &mut syn::Fields,
        variant: Option<Variant>,
    ) -> derive::Result<Self> {
        let attrs = StructAttrs::parse(ctx, attrs);
        let parsed_fields = parse_fields(ctx, setting, &attrs, type_params, fields)?;

        let krate = setting.krate();
        let mut field_index = parsed_fields.len();

        let size_cache = if setting.has_size_cache() && !attrs.no_size_cache {
            Some(add_field(
                fields,
                attrs
                    .size_cache_renamed
                    .or(setting.size_cache_renamed.clone())
                    .map_or("size_cache".to_string(), |(name, _)| name),
                syn::parse_quote!(#krate::rt::SizeCache),
                {
                    field_index += 1;
                    field_index - 1
                },
            ))
        } else {
            None
        };

        let runtime = if setting.has_runtime() {
            Some(add_field(
                fields,
                attrs
                    .runtime_renamed
                    .or(setting.runtime_renamed.clone())
                    .map_or("runtime".to_string(), |(name, _)| name),
                syn::parse_quote!(#krate::rt::RuntimeV2),
                {
                    field_index += 1;
                    field_index - 1
                },
            ))
        } else {
            None
        };

        Ok(Self {
            impler,
            setting,
            type_params,
            fields: parsed_fields,
            size_cache,
            runtime,
            variant,
        })
    }

    pub fn variant(&self) -> Option<&Variant> {
        self.variant.as_ref()
    }

    pub fn size_cache(&self) -> Option<&Field> {
        self.size_cache.as_ref()
    }

    pub fn runtime(&self) -> Option<&Field> {
        self.runtime.as_ref()
    }

    fn trait_bounds(&self, fallback: &'static [&str]) -> &[&str] {
        if self.setting.impl_state() {
            &["StateV2"]
        } else {
            fallback
        }
    }

    pub fn ctor_name(&self) -> syn::Ident {
        match &self.variant {
            Some(variant) => variant.ctor_name(),
            None => format_ident!("empty"),
        }
    }

    pub fn destructure(&self) -> TokenStream {
        let destructure = map_fields!(self, _.destructure_alias());
        quote!(#(#destructure,)*)
    }

    pub fn destructure_prefixed(&self, prefix: impl Into<Option<syn::Ident>>) -> TokenStream {
        let prefix = &prefix.into();
        let destructure = map_fields!(self, _.destructure_alias_prefixed(prefix.clone()));
        quote!(#(#destructure,)*)
    }

    pub fn ctor(&self) -> TokenStream {
        let ctor_name = self.ctor_name();
        let name = self.impler.name();
        let qual = self.variant().map(|variant| variant.qual());
        let mut inits: Vec<_> = map_fields!(self, _.init_default()).collect();

        if let Some(size_cache) = self.size_cache() {
            inits.push(size_cache.init(quote!(SizeCache::new())));
        }

        let (params, set_variant_runtime) = if let Some(runtime) = self.runtime() {
            inits.push(runtime.init(quote!(runtime)));

            (
                Some(quote!(runtime: RuntimeV2)),
                self.variant().map(|variant| {
                    let tag = variant.tag();
                    quote! { let runtime = runtime.nested(#tag); }
                }),
            )
        } else {
            Default::default()
        };

        quote! {
            #[inline]
            pub fn #ctor_name(#params) -> Self {
                #set_variant_runtime
                #name #qual { #(#inits,)* }
            }
        }
    }

    fn impl_ctor(&self) -> TokenStream {
        self.impler
            .impl_with(self.trait_bounds(&["Default"]), self.ctor())
    }

    pub fn setters(&self) -> TokenStream {
        let name = self.impler.name();
        let setters = map_fields!(self, _.setter(name, self.variant()));
        quote!(#(#setters)*)
    }

    fn impl_setters(&self) -> TokenStream {
        self.impler.impl_with(
            self.trait_bounds(if self.variant.is_some() {
                &["Default"]
            } else {
                &[]
            }),
            self.setters(),
        )
    }

    pub fn eq(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let eqs = map_fields!(self, _.eq(is_variant));

        quote! {
            #(#eqs)*
            true
        }
    }

    fn impl_eq(&self) -> TokenStream {
        let eq = self.eq();

        let mut r#impl = self.impler.impl_for(
            "PartialEq",
            quote! {
                fn eq(&self, other: &Self) -> bool {
                    #eq
                }
            },
        );

        r#impl.extend(self.impler.impl_for("Eq", quote!()));
        r#impl
    }

    fn impl_default(&self) -> TokenStream {
        let args = if self.setting.impl_state() {
            Some(quote!(RuntimeV2::default()))
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

    pub fn hasher(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let hashes = map_fields!(self, _.hash(is_variant));
        quote!(#(#hashes)*)
    }

    fn impl_hash(&self) -> TokenStream {
        let hasher = self.hasher();

        self.impler.impl_for(
            "Hash",
            quote! {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    #hasher
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

    pub fn sizer(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let sizers = map_fields!(self, _.sizer(is_variant));
        quote!(#(#sizers)*)
    }

    pub fn serializer(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let serializers = map_fields!(self, _.serializer(is_variant));
        quote!(#(#serializers)*)
    }

    fn impl_serialize(&self) -> TokenStream {
        let sizer = self.sizer();
        let serializer = self.serializer();

        let size_cache = if let Some(size_cache) = &self.size_cache {
            let size_cache = size_cache.field(false);
            quote!(Some(&#size_cache))
        } else {
            quote!(None)
        };

        self.impler.impl_for(
            "SerializeV2",
            quote! {
                fn compute_size_v2(&self) -> u32 {
                    let mut size = 0;
                    #sizer
                    size
                }

                fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    #serializer
                    Ok(())
                }

                #[inline]
                fn size_cache(&self) -> Option<&SizeCache> {
                    #size_cache
                }
            },
        )
    }

    pub fn merger(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let mergers = map_fields!(self, _.merger(is_variant));

        quote! {
            while !reader.eof()? {
                let (field_number, wire_type) = reader.read_tag()?;

                match field_number {
                    #(#mergers,)*
                    _ => reader.skip_field(wire_type)?,
                }
            }
        }
    }

    fn impl_deserialize(&self) -> TokenStream {
        let merger = self.merger();

        self.impler.impl_for(
            "DeserializeV2",
            quote! {
                fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
                    #merger
                    Ok(())
                }
            },
        )
    }

    pub fn runtime_setter(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let runtime_setters = map_fields!(self, _.runtime_setter(is_variant));

        if is_variant {
            quote! {
                #(#runtime_setters)*
                *self_runtime = runtime;
            }
        } else {
            let runtime = self.runtime().unwrap().access();

            quote! {
                #(#runtime_setters)*
                self.#runtime = runtime;
            }
        }
    }

    fn impl_state(&self) -> TokenStream {
        let runtime = self.runtime().unwrap().field(false);
        let runtime_setter = self.runtime_setter();

        self.impler.impl_for(
            "StateV2",
            quote! {
                #[inline]
                fn with_runtime_v2(runtime: RuntimeV2) -> Self {
                    Self::empty(runtime)
                }

                #[inline]
                fn runtime_v2(&self) -> &RuntimeV2 {
                    &#runtime
                }

                fn set_runtime_v2(&mut self, runtime: RuntimeV2) {
                    #runtime_setter
                }
            },
        )
    }

    pub fn meta(&self) -> TokenStream {
        let name = match &self.variant {
            Some(variant) => variant.name(),
            None => self.impler.name(),
        };

        let name = name.to_string();
        let fields = map_fields!(self, _.meta());
        let builtin = self.setting.steit_owned;

        let type_params = if self.variant.is_none() {
            let type_params = self.type_params.iter().map(|type_param| {
                let type_param = type_param.ident.to_string();
                quote!(#type_param)
            });

            Some(quote!(#(#type_params,)*))
        } else {
            None
        };

        quote! {
            StructMeta {
                name: &NameMeta::new(#name),
                type_params: &[#type_params],
                fields: &[#(#fields,)*],
                builtin: #builtin,
            }
        }
    }

    pub fn meta_links(&self) -> TokenStream {
        let links = self.fields.iter().map(|field| {
            let ty = field.ty();
            quote!(<#ty>::LINK)
        });

        quote!(#(#links,)*)
    }

    fn impl_meta(&self) -> TokenStream {
        let meta = self.meta();
        let name = self.impler.name().to_string();

        let param_meta_list = self.type_params.iter().map(|param| {
            let name = &param.ident;
            quote!(FieldTypeMeta::Type(#name::TYPE))
        });

        let links = self.meta_links();

        self.impler.impl_for(
            "HasMeta",
            quote! {
                const NAME: &'static NameMeta = &NameMeta::new(#name);
                const TYPE: &'static TypeMeta = &TypeMeta::Ref(Self::NAME, &[#(#param_meta_list,)*]);
                const LINK: &'static MetaLink = &MetaLink{
                    r#type: Self::TYPE,
                    msg: Some(MessageMeta::Struct(#meta)),
                    links: || &[#links],
                };
            },
        )
    }
}

fn parse_fields<'a>(
    ctx: &Context,
    setting: &'a DeriveSetting,
    attrs: &StructAttrs,
    type_params: &'a [&'a syn::TypeParam],
    fields: &mut syn::Fields,
) -> derive::Result<Vec<DeriveField<'a>>> {
    let mut parsed_fields = Vec::with_capacity(fields.iter().len());

    let reserved_tags: HashSet<_> = attrs.reserved.iter().collect();
    let mut tags = HashSet::new();
    let mut unique_tags = true;

    for (index, field) in fields.iter_mut().enumerate() {
        if let Ok(parsed_field) = DeriveField::parse(ctx, setting, type_params, field, index) {
            let (tag, tag_tokens) = parsed_field.tag_with_tokens();

            if reserved_tags.contains(&tag) {
                ctx.error(tag_tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                ctx.error(tag_tokens, format!("duplicate tag {}", tag));
                unique_tags = false;
            }

            parsed_fields.push(parsed_field);
        }
    }

    if parsed_fields.len() == parsed_fields.capacity() && unique_tags {
        Ok(parsed_fields)
    } else {
        Err(())
    }
}

fn add_field(fields: &mut syn::Fields, name: String, ty: syn::Type, index: usize) -> Field {
    if let syn::Fields::Unit = fields {
        *fields = syn::Fields::Named(syn::parse_quote!({}));
    }

    match fields {
        syn::Fields::Named(fields) => {
            let field = Field::new(Some(format_ident!("{}", name)), ty, index);
            fields.named.extend(field.declare());
            field
        }

        syn::Fields::Unnamed(fields) => {
            let field = Field::new(None, ty, index);
            fields.unnamed.extend(field.declare());
            field
        }

        syn::Fields::Unit => unreachable!("unexpected unit fields"),
    }
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.impl_ctor());
        tokens.extend(self.impl_setters());
        tokens.extend(self.impl_eq());
        tokens.extend(self.impl_default());
        tokens.extend(self.impl_hash());
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
