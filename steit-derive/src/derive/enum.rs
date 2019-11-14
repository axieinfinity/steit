use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{ctx::Context, derive, r#impl::Impl};

use super::{r#struct::Struct, variant::Variant, DeriveSetting};

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
        data: &'a mut syn::DataEnum,
    ) -> derive::Result<Self> {
        if data.variants.is_empty() {
            context.error(&data.variants, "cannot derive for enums with zero variants");
            return Err(());
        }

        Self::parse_variants(setting, context, r#impl, &mut data.variants).map(|variants| Self {
            setting,
            context,
            r#impl,
            variants,
        })
    }

    fn parse_variants(
        setting: &'a DeriveSetting,
        context: &'a Context,
        r#impl: &'a Impl,
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

        for variant in variants.iter_mut() {
            if let Ok((parsed, unknown_attrs)) = Variant::parse(context, variant) {
                parsed_data.push((parsed, unknown_attrs, &mut variant.fields));
            }
        }

        if parsed_data.len() != len {
            return Err(());
        }

        let mut tags = HashSet::new();
        let mut unique = true;
        let mut named = None;

        for (variant, _, fields) in &parsed_data {
            let (tag, tokens) = variant.tag_with_tokens();

            if !tags.insert(tag) {
                context.error(tokens, "duplicate tag");
                unique = false;
            }

            match fields {
                syn::Fields::Named(_) => match named {
                    Some(false) => unreachable!("enum is named and unnamed at the same time"),
                    _ => named = Some(true),
                },

                syn::Fields::Unnamed(_) => match named {
                    Some(true) => unreachable!("enum is named and unnamed at the same time"),
                    _ => named = Some(false),
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
                named,
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

        self.r#impl
            .r#impl(if let Some(default_ctor_name) = default_ctor_name {
                let (declare_arg, call_arg) = match self.setting.no_runtime {
                    false => (quote!(runtime: Runtime), quote!(runtime)),
                    true => (quote!(), quote!()),
                };

                quote! {
                    #[inline]
                    pub fn new(#declare_arg) -> Self {
                        Self::#default_ctor_name(#call_arg)
                    }

                    #(#ctors)*
                }
            } else {
                if self.setting.default(true) {
                    self.context.error(
                        self.r#impl.name(),
                        "expected a variant with tag 0 as the default variant of this enum",
                    );
                }

                quote!(#(#ctors)*)
            })
    }

    fn impl_setters(&self) -> TokenStream {
        let setters = self.variants.iter().map(|variant| variant.setters());
        self.r#impl.r#impl(quote!(#(#setters)*))
    }

    fn impl_default(&self) -> TokenStream {
        let arg = match self.setting.no_runtime {
            false => quote!(Runtime::default()),
            true => quote!(),
        };

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
        let name = self.r#impl.name();

        let runtimes = self.variants.iter().map(|r#struct| {
            let runtime = r#struct
                .runtime()
                .unwrap_or_else(|| unreachable!("expected a `Runtime` field"));

            let variant = r#struct
                .variant()
                .unwrap_or_else(|| unreachable!("expected a variant"));

            let qual = variant.qual();
            // Technically we can use `destructure` and `init` interchangeably here.
            let destructure = runtime.init();

            quote!(#name #qual { #destructure, .. } => runtime)
        });

        self.r#impl.r#impl_for(
            "Runtimed",
            quote! {
                #[inline]
                fn with_runtime(runtime: Runtime) -> Self {
                    Self::new(runtime)
                }

                #[inline]
                fn runtime(&self) -> &Runtime {
                    match self { #(#runtimes,)*}
                }
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
                    size += #tag.size();
                    #sizer
                }
            }
        });

        let mut sizer = quote! {
            let mut size = 0;
            match self { #(#sizers)* }
            size
        };

        if !self.setting.no_runtime {
            sizer = quote! {
                self.runtime().get_or_set_cached_size_from(|| {
                    #sizer
                })
            }
        }

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
                fn size(&self) -> u32 {
                    #sizer
                }

                fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
                    match self { #(#serializers)* }
                    Ok(())
                }
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

            let arg = match self.setting.no_runtime {
                false => quote!(self.runtime().parent()),
                true => quote!(),
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

        self.r#impl.impl_for(
            "Merge",
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
                fn is_root(&self) -> bool {
                    self.runtime().parent().is_root()
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
                                "`add` and `remove` are not supported on structs and enums",
                            )),
                        }
                    }
                }
            },
        )
    }
}

impl<'a> ToTokens for Enum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.setting.ctors(true) {
            tokens.extend(self.impl_ctors());
        }

        if self.setting.setters() {
            tokens.extend(self.impl_setters());
        }

        if self.setting.default(true) {
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
            tokens.extend(self.impl_state());
        }
    }
}
