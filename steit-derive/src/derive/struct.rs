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
    reserved_tags: Vec<u32>,

    no_size_cache: bool,

    size_cache_renamed: Option<(String, TokenStream)>,
    runtime_renamed: Option<(String, TokenStream)>,

    csharp_name: Option<String>,
}

impl StructAttrs {
    pub fn parse(ctx: &Context, attrs: impl AttributeParse) -> Self {
        let mut reserved_tags = VecAttribute::new(ctx, "reserved_tags");

        let mut no_size_cache = Attribute::new(ctx, "no_size_cache");

        let mut size_cache_renamed = Attribute::new(ctx, "size_cache_renamed");
        let mut runtime_renamed = Attribute::new(ctx, "runtime_renamed");

        let mut csharp_name = Attribute::new(ctx, "csharp_name");

        attrs.parse(ctx, true, |meta| match meta {
            syn::Meta::List(meta) if reserved_tags.parse_int_list(meta) => true,

            syn::Meta::Path(path) if no_size_cache.parse_path(path) => true,
            syn::Meta::NameValue(meta) if no_size_cache.parse_bool(meta) => true,

            syn::Meta::NameValue(meta) if size_cache_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,

            syn::Meta::NameValue(meta) if csharp_name.parse_str(meta) => true,

            _ => false,
        });

        Self {
            reserved_tags: reserved_tags.get(),

            no_size_cache: no_size_cache.get().unwrap_or_default(),

            size_cache_renamed: size_cache_renamed.get_with_tokens(),
            runtime_renamed: runtime_renamed.get_with_tokens(),

            csharp_name: csharp_name.get(),
        }
    }
}

pub struct Struct<'a> {
    impler: &'a Implementer<'a>,
    setting: &'a DeriveSetting,
    attrs: StructAttrs,
    type_params: &'a [&'a syn::TypeParam],
    fields: Vec<DeriveField<'a>>,
    size_cache: Option<Field>,
    runtime: Option<Field>,
    variant: Option<Variant<'a>>,
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
        variant: Option<Variant<'a>>,
    ) -> derive::Result<Self> {
        let attrs = StructAttrs::parse(ctx, attrs);
        let parsed_fields = parse_fields(ctx, setting, &attrs, type_params, fields)?;

        let krate = setting.krate();
        let mut field_index = parsed_fields.len();

        let size_cache = if setting.has_size_cache && !attrs.no_size_cache {
            Some(add_field(
                fields,
                match (&attrs.size_cache_renamed, &setting.size_cache_renamed) {
                    (Some((name, _)), _) => name.clone(),
                    (_, Some((name, _))) => name.clone(),
                    _ => "size_cache".to_string(),
                },
                syn::parse_quote!(#krate::rt::SizeCache),
                {
                    field_index += 1;
                    field_index - 1
                },
            ))
        } else {
            None
        };

        let runtime = if setting.has_runtime {
            Some(add_field(
                fields,
                match (&attrs.runtime_renamed, &setting.runtime_renamed) {
                    (Some((name, _)), _) => name.clone(),
                    (_, Some((name, _))) => name.clone(),
                    _ => "runtime".to_string(),
                },
                syn::parse_quote!(#krate::rt::Runtime),
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
            attrs,
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
        if self.setting.derive_state {
            &["State"]
        } else {
            fallback
        }
    }

    pub fn ctor_name(&self) -> syn::Ident {
        match &self.variant {
            Some(variant) => variant.ctor_name(),
            None => format_ident!("{}", &self.setting.ctor_prefix),
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
                Some(quote!(runtime: Runtime)),
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

    fn impl_partial_eq(&self) -> TokenStream {
        let eq = self.eq();

        self.impler.impl_for(
            "PartialEq",
            quote! {
                fn eq(&self, other: &Self) -> bool {
                    #eq
                }
            },
        )
    }

    fn impl_eq(&self) -> TokenStream {
        self.impler.impl_for("Eq", quote!())
    }

    fn impl_default(&self) -> TokenStream {
        let ctor_name = self.ctor_name();

        let args = if self.setting.derive_state {
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
                    Self::#ctor_name(#args)
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
                const WIRE_TYPE: WireType = WireType::Sized;
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
            "Serialize",
            quote! {
                fn compute_size(&self) -> u32 {
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
            "Deserialize",
            quote! {
                fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
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

    pub fn replayer(&self) -> TokenStream {
        let name = self.impler.name().to_token_stream().to_string();
        let is_variant = self.variant.is_some();
        let replayers = map_fields!(self, _.replayer(is_variant));

        let update = if is_variant {
            quote! {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`LogEntryKind::Update` is not supported on variants but their enums",
                ))
            }
        } else {
            quote!(self.handle_update(reader))
        };

        quote! {
            if let Some(tag) = path.next() {
                match tag {
                    #(#replayers,)*

                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("unexpected tag {}", tag),
                    )),
                }
            } else {
                match kind {
                    LogEntryKind::Update => #update,

                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{:?} is not supported on `{}`", kind, #name),
                    )),
                }
            }
        }
    }

    fn impl_state(&self) -> TokenStream {
        let ctor_name = self.ctor_name();
        let runtime = self.runtime().unwrap().field(false);
        let runtime_setter = self.runtime_setter();
        let replayer = self.replayer();

        self.impler.impl_for(
            "State",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::#ctor_name(runtime)
                }

                #[inline]
                fn runtime(&self) -> &Runtime {
                    &#runtime
                }

                fn set_runtime(&mut self, runtime: Runtime) {
                    #runtime_setter
                }

                fn handle(
                    &mut self,
                    mut path: impl Iterator<Item = u32>,
                    kind: LogEntryKind,
                    key: Option<u32>,
                    reader: &mut Reader<impl io::Read>,
                ) -> io::Result<()> {
                    #replayer
                }
            },
        )
    }

    pub fn meta(&self) -> TokenStream {
        let rust_name = match &self.variant {
            Some(variant) => variant.name().to_string(),
            None => self.impler.name().to_string(),
        };

        let csharp_name = match &self.attrs.csharp_name {
            Some(csharp_name) => quote!(Some(#csharp_name)),
            None => quote!(None),
        };

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
                name: &NameMeta {
                    rust: #rust_name,
                    csharp: #csharp_name,
                },
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
        let rust_name = self.impler.name().to_string();

        let csharp_name = match &self.attrs.csharp_name {
            Some(csharp_name) => quote!(Some(#csharp_name)),
            None => quote!(None),
        };

        let param_meta_list = self.type_params.iter().map(|param| {
            let name = &param.ident;
            quote!(FieldTypeMeta::Type(#name::TYPE))
        });

        let links = self.meta_links();

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

    let reserved_tags: HashSet<_> = attrs.reserved_tags.iter().collect();
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
        if self.setting.derive_ctors {
            tokens.extend(self.impl_ctor());
        }

        if self.setting.derive_setters {
            tokens.extend(self.impl_setters());
        }

        if self.setting.derive_partial_eq {
            tokens.extend(self.impl_partial_eq());
        }

        if self.setting.derive_eq {
            tokens.extend(self.impl_eq());
        }

        if self.setting.derive_default {
            tokens.extend(self.impl_default());
        }

        if self.setting.derive_hash {
            tokens.extend(self.impl_hash());
        }

        if self.setting.derive_wire_type {
            tokens.extend(self.impl_wire_type());
        }

        if self.setting.derive_serialize {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.derive_deserialize {
            tokens.extend(self.impl_deserialize());
        }

        if self.setting.derive_state {
            tokens.extend(self.impl_state());
        }

        if self.setting.derive_meta {
            tokens.extend(self.impl_meta());
        }
    }
}
