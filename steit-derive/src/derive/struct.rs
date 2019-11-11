use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    attr::{Attr, AttrParse},
    ctx::Context,
    derive,
    r#impl::Impl,
};

use super::{
    field::{Field, Runtime},
    variant::Variant,
    DeriveSetting,
};

struct StructAttrs {
    runtime_renamed: Option<(String, TokenStream)>,
}

impl StructAttrs {
    pub fn parse(context: &Context, attrs: impl AttrParse) -> Self {
        let mut runtime_renamed = Attr::new(context, "runtime_renamed");

        attrs.parse(context, true, &mut |meta| match meta {
            syn::Meta::NameValue(meta) if runtime_renamed.parse_str(meta) => true,
            _ => false,
        });

        Self {
            runtime_renamed: runtime_renamed.get_with_tokens(),
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
    r#impl: &'a Impl<'a>,
    fields: Vec<Field<'a>>,
    runtime: Option<Runtime<'a>>,
    variant: Option<Variant>,
}

impl<'a> Struct<'a> {
    pub fn parse(
        setting: &'a DeriveSetting,
        context: &'a Context,
        r#impl: &'a Impl<'a>,
        attrs: impl AttrParse,
        fields: &'a mut syn::Fields,
        variant: impl Into<Option<Variant>>,
    ) -> derive::Result<Self> {
        let attrs = StructAttrs::parse(context, attrs);

        Self::parse_fields(setting, context, fields).and_then(|parsed| {
            let runtime = if !setting.no_runtime {
                if let syn::Fields::Unit = fields {
                    *fields = syn::Fields::Named(syn::parse_quote!({}));
                }

                Some(match fields {
                    syn::Fields::Named(fields) => {
                        let name = match attrs.runtime_renamed {
                            Some((runtime_renamed, _)) => runtime_renamed,
                            None => "runtime".to_owned(),
                        };

                        let name = format_ident!("{}", name);
                        let runtime = Runtime::new(setting, name, parsed.len());
                        fields.named.extend(runtime.declare());
                        runtime
                    }

                    syn::Fields::Unnamed(fields) => {
                        if let Some((runtime_renamed, _)) = attrs.runtime_renamed {
                            context.error(
                                &fields,
                                format!(
                                    "unexpected {} on unnamed fields",
                                    format!("#[steit(runtime_renamed = {:?})]", runtime_renamed),
                                ),
                            );
                        }

                        let runtime = Runtime::new(setting, None, parsed.len());
                        fields.unnamed.extend(runtime.declare());
                        runtime
                    }

                    syn::Fields::Unit => unreachable!("unexpected unit fields"),
                })
            } else {
                if let Some((_, runtime_renamed_tokens)) = attrs.runtime_renamed {
                    context.error(
                        runtime_renamed_tokens,
                        "this has no effect because #[steit(no_runtime)] is set",
                    );
                }

                None
            };

            Ok(Self {
                setting,
                r#impl,
                fields: parsed,
                runtime,
                variant: variant.into(),
            })
        })
    }

    fn parse_fields(
        setting: &'a DeriveSetting,
        context: &Context,
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

    pub fn runtime(&self) -> Option<&Runtime> {
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

    pub fn ctor(&self) -> TokenStream {
        let ctor_name = self.ctor_name();
        let name = self.r#impl.name();

        let qual = self.variant.as_ref().map(|variant| variant.qual());
        let mut inits: Vec<_> = map_fields!(self, init).collect();

        let (arg, runtime) = match &self.runtime {
            Some(runtime) => {
                inits.push(runtime.init());

                (
                    quote!(runtime: Runtime),
                    self.variant.as_ref().map(|variant| {
                        let tag = variant.tag();
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
        let arg = self.runtime.as_ref().map(|_| quote!(Default::default()));

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

    fn impl_runtimed(&self) -> TokenStream {
        let runtime = self
            .runtime
            .as_ref()
            .unwrap_or_else(|| unreachable!("expected a `Runtime` field"))
            .access();

        self.r#impl.r#impl_for(
            "Runtimed",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::new(runtime)
                }

                #[inline]
                fn runtime(&self) -> &Runtime {
                    &self.#runtime
                }
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

        let mut sizer = quote! {
            let mut size = 0;
            #sizer
            size
        };

        if self.runtime.is_some() {
            sizer = quote! {
                self.runtime().get_or_set_cached_size_from(|| {
                    #sizer
                })
            }
        }

        self.r#impl.impl_for(
            "Serialize",
            quote! {
                fn size(&self) -> u32 {
                    #sizer
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
                let key = u32::deserialize(reader)?;
                let (tag, wire_type) = wire_type::split_key(key);

                match tag {
                    #(#mergers)*
                    _ => { de::exhaust_nested(tag, wire_type, reader)?; }
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
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // This case is up to `Enum` to handle.
        if self.variant.is_some() {
            panic!("unexpected variant");
        }

        if self.setting.ctors(false) {
            tokens.extend(self.impl_ctor());
        }

        if self.setting.setters() {
            tokens.extend(self.impl_setters());
        }

        if self.setting.default(false) {
            tokens.extend(self.impl_default());
        }

        tokens.extend(self.impl_wire_type());

        if self.setting.runtimed() {
            tokens.extend(self.impl_runtimed());
        }

        if self.setting.serialize {
            tokens.extend(self.impl_serialize());
        }

        if self.setting.merge {
            tokens.extend(self.impl_merge());
        }

        if self.setting.state {
            // tokens.extend(self.impl_state());
        }
    }
}
