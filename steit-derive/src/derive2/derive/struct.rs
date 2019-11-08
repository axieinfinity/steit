use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::derive2::{
    attr::{Attr, AttrParse},
    ctx::Context,
    derive,
    r#impl::Impl,
};

use super::{
    field::{Field, Runtime},
    variant::Variant,
    DeriveKind,
};

struct StructAttrs {
    runtime_renamed: Option<String>,
}

impl StructAttrs {
    pub fn parse(context: &Context, attrs: impl AttrParse) -> Self {
        let mut runtime_renamed = Attr::new(context, "runtime_renamed");

        attrs.parse(context, true, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,
            _ => false,
        });

        Self {
            runtime_renamed: runtime_renamed.get(),
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
    derive: &'a DeriveKind,
    context: &'a Context,
    r#impl: &'a Impl<'a>,
    fields: Vec<Field<'a>>,
    runtime: Runtime,
    variant: Option<Variant>,
}

impl<'a> Struct<'a> {
    pub fn parse(
        derive: &'a DeriveKind,
        context: &'a Context,
        r#impl: &'a Impl<'a>,
        attrs: impl AttrParse,
        fields: &'a mut syn::Fields,
        variant: impl Into<Option<Variant>>,
    ) -> derive::Result<Self> {
        let attrs = StructAttrs::parse(context, attrs);

        Self::parse_fields(derive, context, fields).and_then(|parsed| {
            if let syn::Fields::Unit = fields {
                *fields = syn::Fields::Named(syn::parse_quote!({}));
            }

            let runtime = match fields {
                syn::Fields::Named(fields) => {
                    let name = attrs.runtime_renamed.unwrap_or("runtime".to_owned());
                    let name = format_ident!("{}", name);
                    let runtime = Runtime::new(name, parsed.len());
                    fields.named.extend(runtime.declare());
                    runtime
                }

                syn::Fields::Unnamed(fields) => {
                    if let Some(runtime_renamed) = attrs.runtime_renamed {
                        context.error(
                            &fields,
                            format!(
                                "unexpected {} on unnamed fields",
                                format!("#[steit(runtime_renamed = {:?})]", runtime_renamed),
                            ),
                        );
                    }

                    let runtime = Runtime::new(None, parsed.len());
                    fields.unnamed.extend(runtime.declare());
                    runtime
                }

                syn::Fields::Unit => unreachable!("unexpected unit fields"),
            };

            Ok(Self {
                derive,
                context,
                r#impl,
                fields: parsed,
                runtime,
                variant: variant.into(),
            })
        })
    }

    fn parse_fields(
        derive: &'a DeriveKind,
        context: &Context,
        fields: &mut syn::Fields,
    ) -> derive::Result<Vec<Field<'a>>> {
        let len = fields.iter().len();
        let mut parsed = Vec::with_capacity(len);

        for (index, field) in fields.iter_mut().enumerate() {
            if let Ok(field) = Field::parse(derive, context, field, index) {
                parsed.push(field);
            }
        }

        if parsed.len() != len {
            return Err(());
        }

        let mut tags = HashSet::new();
        let mut unique = true;

        for field in &parsed {
            let (tag, tokens) = field.tag_with_tokens();

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

    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
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

    pub fn ctor(&self) -> TokenStream {
        let ctor_name = self.ctor_name();

        let name = self.r#impl.name();
        let qual = self.variant.as_ref().map(|variant| variant.qual());

        let mut inits: Vec<_> = map_fields!(self, init).collect();
        inits.push(self.runtime.init());

        quote! {
            #[inline]
            pub fn #ctor_name(runtime: Runtime2) -> Self {
                #name #qual { #(#inits,)* }
            }
        }
    }

    fn impl_ctor(&self) -> TokenStream {
        self.r#impl.r#impl(self.ctor())
    }

    fn impl_default(&self) -> TokenStream {
        self.r#impl.impl_for(
            "Default",
            quote! {
                #[inline]
                fn default() -> Self {
                    Self::new(Default::default())
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

        self.r#impl.impl_for(
            "Serialize2",
            quote! {
                fn size(&self) -> u32 {
                    let mut size = 0;
                    #sizer
                    size
                }

                fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    #serializer
                    Ok(())
                }
            },
        )
    }

    pub fn merger(&self) -> TokenStream {
        let is_variant = self.variant.is_some();
        let mergers = map_fields!(self, merger(is_variant));

        quote! {
            while !reader.eof()? {
                // TODO: Remove `as Deserialize` after refactoring `Varint`
                let key = <u32 as Deserialize2>::deserialize(reader)?;
                let (tag, wire_type) = wire_type::split_key(key);

                match tag {
                    #(#mergers)*
                    _ => { de2::exhaust_nested(tag, wire_type, reader)?; }
                }
            }
        }
    }

    fn impl_deserialize(&self) -> TokenStream {
        let merger = self.merger();

        self.r#impl.impl_for(
            "Deserialize2",
            quote! {
                fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
                    #merger
                    Ok(())
                }
            },
        )
    }
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // This case is up to `Enum` to handle.
        if self.variant.is_some() {
            return;
        }

        tokens.extend(self.impl_ctor());
        tokens.extend(self.impl_default());
        tokens.extend(self.impl_wire_type());
        tokens.extend(self.impl_serialize());
        tokens.extend(self.impl_deserialize());
    }
}
