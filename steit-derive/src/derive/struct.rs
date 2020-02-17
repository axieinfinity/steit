use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse, VecAttr},
    ctx::Context,
    r#impl::Impl,
};

use super::{
    derive::{self, DeriveSetting},
    field::{ExtraField, Field},
    variant::Variant,
};

struct StructAttrs {
    cached_size_renamed: Option<(String, TokenStream)>,
    runtime_renamed: Option<(String, TokenStream)>,
    reserved: Vec<u16>,
}

impl StructAttrs {
    pub fn parse(context: &Context, attrs: impl AttrParse) -> Self {
        let mut cached_size_renamed = Attr::new(context, "cached_size_renamed");
        let mut runtime_renamed = Attr::new(context, "runtime_renamed");
        let mut reserved = VecAttr::new(context, "reserved");

        attrs.parse(context, true, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if cached_size_renamed.parse_str(meta) => true,
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,
            syn::Meta::List(meta) if reserved.parse_int_list(meta) => true,
            _ => false,
        });

        Self {
            cached_size_renamed: cached_size_renamed.get_with_tokens(),
            runtime_renamed: runtime_renamed.get_with_tokens(),
            reserved: reserved.get(),
        }
    }
}

macro_rules! map_fields {
    ($struct:ident, $method:ident) => {
        $struct.fields.iter().map(|field| field.$method())
    };

    ($struct:ident, $method:ident ($($rest:tt)*)) => {
        $struct.fields.iter().map(|field| field.$method($($rest)*))
    };
}

pub struct Struct<'a> {
    setting: &'a DeriveSetting,
    context: &'a Context,
    r#impl: &'a Impl<'a>,
    fields: Vec<Field<'a>>,
    cached_size: Option<ExtraField>,
    runtime: Option<ExtraField>,
    variant: Option<Variant>,
}

impl<'a> Struct<'a> {
    pub fn parse(
        setting: &'a DeriveSetting,
        context: &'a Context,
        r#impl: &'a Impl<'a>,
        attrs: impl AttrParse,
        fields: &'a mut syn::Fields,
        named_hint: Option<bool>,
        variant: impl Into<Option<Variant>>,
    ) -> derive::Result<Self> {
        let attrs = StructAttrs::parse(context, attrs);

        Self::parse_fields(setting, context, &attrs, fields).and_then(|parsed| {
            let mut index = parsed.len();

            let cached_size = if setting.cached_size() {
                let krate = setting.krate();

                Some(Self::add_field(
                    context,
                    fields,
                    named_hint,
                    attrs.cached_size_renamed,
                    "cached_size",
                    syn::parse_quote!(#krate::CachedSize),
                    {
                        index += 1;
                        index - 1
                    },
                ))
            } else {
                if let Some((_, tokens)) = &attrs.runtime_renamed {
                    context.error(
                        tokens,
                        "this has no effect because #[steit(no_cached_size)] was set",
                    );
                }

                None
            };

            let runtime = if setting.runtime() {
                let krate = setting.krate();

                Some(Self::add_field(
                    context,
                    fields,
                    named_hint,
                    attrs.runtime_renamed,
                    "runtime",
                    syn::parse_quote!(#krate::Runtime),
                    {
                        index += 1;
                        index - 1
                    },
                ))
            } else {
                if let Some((_, tokens)) = &attrs.runtime_renamed {
                    context.error(
                        tokens,
                        "this has no effect because the current object is not a `State`",
                    );
                }

                None
            };

            Ok(Self {
                setting,
                context,
                r#impl,
                fields: parsed,
                cached_size,
                runtime,
                variant: variant.into(),
            })
        })
    }

    fn parse_fields(
        setting: &'a DeriveSetting,
        context: &Context,
        attrs: &StructAttrs,
        fields: &mut syn::Fields,
    ) -> derive::Result<Vec<Field<'a>>> {
        let len = fields.iter().len();
        let mut parsed = Vec::with_capacity(len);

        for (index, field) in fields.iter_mut().enumerate() {
            if let Ok(field) = Field::parse(setting, context, field, index) {
                parsed.push(field);
            }
        }

        if parsed.len() != len {
            return Err(());
        }

        let reserved: HashSet<_> = attrs.reserved.iter().collect();
        let mut tags = HashSet::new();
        let mut unique = true;

        for field in &parsed {
            let (tag, tokens) = field.tag_with_tokens();

            if reserved.contains(&tag) {
                context.error(tokens, format!("tag {} has been reserved", tag));
            }

            if !tags.insert(tag) {
                context.error(tokens, "duplicate tag");
                unique = false;
            }
        }

        if unique {
            Ok(parsed)
        } else {
            Err(())
        }
    }

    fn add_field(
        context: &Context,
        fields: &mut syn::Fields,
        named_hint: Option<bool>,
        renamed: Option<(String, TokenStream)>,
        default_name: &str,
        ty: syn::Type,
        index: usize,
    ) -> ExtraField {
        if let syn::Fields::Unit = fields {
            match named_hint {
                Some(true) | None => *fields = syn::Fields::Named(syn::parse_quote!({})),
                Some(false) => *fields = syn::Fields::Unnamed(syn::parse_quote!(())),
            }
        }

        match fields {
            syn::Fields::Named(fields) => {
                let name = match renamed {
                    Some((name, _)) => name,
                    None => default_name.to_owned(),
                };

                let name = format_ident!("{}", name);
                let field = ExtraField::new(name, ty, index);
                fields.named.extend(field.declare());
                field
            }

            syn::Fields::Unnamed(fields) => {
                if let Some((name, _)) = renamed {
                    context.error(
                        &fields,
                        format!(
                            "unexpected {} on unnamed fields",
                            format!("#[steit({}_renamed = {:?})]", default_name, name),
                        ),
                    );
                }

                let field = ExtraField::new(None, ty, index);
                fields.unnamed.extend(field.declare());
                field
            }

            syn::Fields::Unit => unreachable!("unexpected unit fields"),
        }
    }

    pub fn cached_size(&self) -> Option<&ExtraField> {
        self.cached_size.as_ref()
    }

    pub fn runtime(&self) -> Option<&ExtraField> {
        self.runtime.as_ref()
    }

    pub fn variant(&self) -> Option<&Variant> {
        self.variant.as_ref()
    }

    pub fn ctor_name(&self) -> syn::Ident {
        match &self.variant {
            Some(variant) => variant.ctor_name(),
            None => format_ident!("new"),
        }
    }

    pub fn destructure(&self) -> TokenStream {
        let destructure = map_fields!(self, destructure);
        quote!(#(#destructure,)*)
    }

    pub fn ctor(&self) -> TokenStream {
        let ctor_name = self.ctor_name();
        let name = self.r#impl.name();

        let qual = self.variant.as_ref().map(|r#struct| r#struct.qual());
        let mut inits: Vec<_> = map_fields!(self, init).collect();

        match &self.cached_size {
            Some(cached_size) => inits.push(cached_size.init(quote!(CachedSize::new()))),
            None => (),
        }

        let (arg, runtime) = match &self.runtime {
            Some(runtime) => {
                inits.push(runtime.init(quote!(runtime)));

                (
                    quote!(runtime: Runtime),
                    self.variant.as_ref().map(|r#struct| {
                        let tag = r#struct.tag();
                        quote! { let runtime = runtime.nested(#tag); }
                    }),
                )
            }

            None => (quote!(), None),
        };

        quote! {
            #[inline]
            pub fn #ctor_name(#arg) -> Self {
                #runtime
                #name #qual { #(#inits,)* }
            }
        }
    }

    fn impl_ctor(&self) -> TokenStream {
        self.r#impl.r#impl(self.ctor())
    }

    pub fn setters(&self) -> TokenStream {
        let name = self.r#impl.name();
        let setters = map_fields!(self, setter(name, self.variant()));
        quote!(#(#setters)*)
    }

    fn impl_setters(&self) -> TokenStream {
        let setters = self.setters();
        self.r#impl.r#impl(quote!(#setters))
    }

    fn impl_default(&self) -> TokenStream {
        let arg = self.runtime.as_ref().map(|_| quote!(Runtime::default()));

        self.r#impl.impl_for(
            "Default",
            quote! {
                #[inline]
                fn default() -> Self {
                    Self::new(#arg)
                }
            },
        )
    }

    fn impl_wire_type(&self) -> TokenStream {
        self.r#impl.impl_for(
            "WireType",
            quote! {
                const WIRE_TYPE: u8 = 2;
            },
        )
    }

    pub fn sizer(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let sizers = map_fields!(self, sizer(is_variant));
        quote!(#(#sizers)*)
    }

    pub fn serializer(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let serializers = map_fields!(self, serializer(is_variant));
        quote!(#(#serializers)*)
    }

    fn impl_serialize(&self) -> TokenStream {
        let sizer = self.sizer();
        let serializer = self.serializer();

        let (set_cached_size, cached_size) = match &self.cached_size {
            Some(cached_size) => {
                let access = cached_size.access();

                (
                    quote! { self.#access.set(size); },
                    quote! {
                        #[inline]
                        fn cached_size(&self) -> u32 {
                            self.#access.get()
                        }
                    },
                )
            }

            None => (quote!(), quote!()),
        };

        self.r#impl.impl_for(
            "Serialize",
            quote! {
                fn compute_size(&self) -> u32 {
                    let mut size = 0;
                    #sizer
                    #set_cached_size
                    size
                }

                fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    #serializer
                    Ok(())
                }

                #cached_size
            },
        )
    }

    pub fn merger(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let mergers = map_fields!(self, merger(is_variant));

        quote! {
            while !reader.eof()? {
                let key = u32::deserialize(reader)?;
                let (tag, wire_type) = wire_type::split_key(key);

                match tag {
                    #(#mergers)*
                    _ => { exhaust_nested(tag, wire_type, reader)?; }
                }
            }
        }
    }

    fn impl_merge(&self) -> TokenStream {
        let merger = self.merger();

        self.r#impl.impl_for(
            "Merge",
            quote! {
                fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
                    #merger
                    Ok(())
                }
            },
        )
    }

    pub fn replayer(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let replayers = map_fields!(self, replayer(is_variant));

        let update = if is_variant {
            quote! {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`update` is not supported on variants (but enums)",
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
                    ReplayKind::Update => #update,

                    ReplayKind::Add | ReplayKind::Remove => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "`add` and `remove` are not supported on structs and variants",
                    )),
                }
            }
        }
    }

    fn impl_state(&self) -> TokenStream {
        let runtime = self
            .runtime
            .as_ref()
            .unwrap_or_else(|| unreachable!("expected a `Runtime` field"))
            .access();

        let replayer = self.replayer();

        self.r#impl.impl_for(
            "State",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::new(runtime)
                }

                #[inline]
                fn runtime(&self) -> &Runtime {
                    &self.#runtime
                }

                #[inline]
                fn handle<'a>(
                    &mut self,
                    path: &mut impl Iterator<Item = &'a u16>,
                    kind: &ReplayKind,
                    reader: &mut Eof<impl io::Read>,
                ) -> io::Result<()> {
                    #replayer
                }
            },
        )
    }

    pub fn meta(&self) -> TokenStream {
        let name = match &self.variant {
            Some(variant) => variant.name(),
            None => self.r#impl.name(),
        };

        let name = name.to_token_stream().to_string();
        let fields = map_fields!(self, meta);

        quote! {
            &Struct {
                name: #name,
                fields: &[#(#fields,)*],
            }
        }
    }

    fn impl_meta(&self) -> TokenStream {
        let meta = self.meta();
        let name = self.r#impl.name().to_token_stream().to_string();

        self.r#impl.impl_for(
            "HasMeta",
            quote! {
                const META: &'static Meta = &Meta::Struct(#meta);
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

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // This case is up to `Enum` to handle.
        if self.variant.is_some() {
            panic!("unexpected variant");
        }

        if self.setting.ctors(self.context, false) {
            tokens.extend(self.impl_ctor());
        }

        if self.setting.setters(self.context) {
            tokens.extend(self.impl_setters());
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
