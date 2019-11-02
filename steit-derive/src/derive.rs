use crate::{context::Context, r#struct::Struct, util};

#[derive(PartialEq)]
pub enum DeriveKind {
    Serialize,
    Deserialize,
    State,
}

pub fn derive(kind: &DeriveKind, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let context = Context::new();

    let output = match input.data {
        syn::Data::Enum(ref data) => impl_enum(&context, kind, &input, data),

        syn::Data::Struct(ref data) => {
            impl_struct(&context, kind, &input, &data.struct_token, &data.fields)
        }

        syn::Data::Union(ref data) => impl_union(&context, kind, &input, data),
    };

    let output = if let Err(errors) = context.check() {
        to_compile_errors(errors)
    } else {
        wrap_in_const(kind, &input.ident, output.unwrap_or_default())
    };

    output.into()
}

macro_rules! map_fields {
    ($struct:ident, $method:ident) => {
        $struct.indexed().iter().map(|field| field.$method())
    };

    ($struct:ident, $method:ident ($($rest:tt)*)) => {
        $struct.indexed().iter().map(|field| field.$method($($rest)*))
    };
}

fn impl_enum(
    context: &Context,
    kind: &DeriveKind,
    input: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> Result<proc_macro2::TokenStream, ()> {
    if data.variants.is_empty() {
        context.error(&data.variants, "cannot derive for enums with zero variants");
        return Err(());
    }

    data.variants
        .iter()
        .map(|variant| {
            if variant.discriminant.is_some() {
                context.error(&data.variants, "cannot derive for enums with discriminants");
                return Err(());
            }

            parse_struct(
                context,
                kind,
                input,
                &variant.ident,
                &variant.fields,
                Some(variant),
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|variants| {
            let name = &input.ident;
            let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

            let ctors_and_setters = variants
                .iter()
                .map(|r#struct| r#struct.get_ctor_and_setters());

            let sizer_matches = variants.iter().map(|r#struct| {
                let qual = r#struct.qual();

                let tag = r#struct
                    .tag()
                    .unwrap_or_else(|| unreachable!("expected a tag number"));

                let destructuring = map_fields!(r#struct, get_destructuring);
                let sizers = map_fields!(r#struct, get_sizer(true));

                quote! {
                    #name #qual { #(#destructuring)*, .. } => {
                        size += #tag.size();
                        #(#sizers)*
                    },
                }
            });

            let serializer_matches = variants.iter().map(|r#struct| {
                let qual = r#struct.qual();

                let tag = r#struct
                    .tag()
                    .unwrap_or_else(|| unreachable!("expected a tag number"));

                let destructuring = map_fields!(r#struct, get_destructuring);
                let serializers = map_fields!(r#struct, get_serializer(true));

                quote! {
                    #name #qual { #(#destructuring)*, .. } => {
                        #tag.serialize(writer)?;
                        #(#serializers)*
                    },
                }
            });

            let r#impl = quote! {
                #(#ctors_and_setters)*

                impl #impl_generics Serialize for #name #ty_generics #where_clause {
                    fn size(&self) -> u32 {
                        let mut size = 0;
                        match self { #(#sizer_matches)* }
                        size
                    }

                    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                        self.size().serialize(writer)?;
                        match self { #(#serializer_matches)* }
                        Ok(())
                    }
                }
            };

            if kind == &DeriveKind::State {
                let runtime_matches = variants.iter().map(|r#struct| {
                    let qual = r#struct.qual();

                    let runtime = r#struct
                        .runtime()
                        .unwrap_or_else(|| unreachable!("expected a `Runtime` field"))
                        .get_access();

                    quote!(#name #qual { #runtime: ref runtime, .. } => runtime,)
                });

                quote! {
                    impl #impl_generics #name #ty_generics #where_clause {
                        fn runtime(&self) -> &Runtime {
                            match self {
                                #(#runtime_matches)*
                            }
                        }
                    }

                    #r#impl
                }
            } else {
                r#impl
            }
        })
}

fn impl_struct<'a, O: quote::ToTokens>(
    context: &Context,
    kind: &DeriveKind,
    input: &'a syn::DeriveInput,
    object: O,
    fields: &'a syn::Fields,
) -> Result<proc_macro2::TokenStream, ()> {
    parse_struct(context, kind, input, object, fields, None).map(|r#struct| {
        let name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let ctor_and_setters = r#struct.get_ctor_and_setters();

        let sizer_and_serializer =
            r#struct
                .get_sizer_and_serializer()
                .map(|(sizer, serializer)| {
                    quote! {
                        impl #impl_generics Serialize for #name #ty_generics #where_clause {
                            fn size(&self) -> u32 {
                                let mut size = 0;
                                #sizer
                                size
                            }

                            fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                                self.size().serialize(writer)?;
                                #serializer
                                Ok(())
                            }
                        }
                    }
                });

        let r#impl = quote! {
            #ctor_and_setters
            #sizer_and_serializer
        };

        if kind == &DeriveKind::State {
            let runtime = r#struct
                .runtime()
                .unwrap_or_else(|| unreachable!("expected a `Runtime` field"))
                .get_access();

            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #[inline]
                    fn runtime(&self) -> &Runtime {
                        &self.#runtime
                    }
                }

                #r#impl
            }
        } else {
            r#impl
        }
    })
}

fn impl_union(
    context: &Context,
    _kind: &DeriveKind,
    _input: &syn::DeriveInput,
    data: &syn::DataUnion,
) -> Result<proc_macro2::TokenStream, ()> {
    context.error(data.union_token, "cannot derive for unions yet");
    Err(())
}

fn parse_struct<'a, O: quote::ToTokens>(
    context: &Context,
    kind: &DeriveKind,
    input: &'a syn::DeriveInput,
    object: O,
    fields: &'a syn::Fields,
    variant: Option<&'a syn::Variant>,
) -> Result<Struct<'a>, ()> {
    let r#impl = |fields: &'a syn::punctuated::Punctuated<_, _>| {
        Struct::parse(&context, kind, &input, &object, fields, variant)
    };

    match *fields {
        syn::Fields::Named(ref fields) => r#impl(&fields.named),
        syn::Fields::Unnamed(ref fields) => r#impl(&fields.unnamed),
        syn::Fields::Unit => {
            context.error(object, "cannot derive for unit structs");
            Err(())
        }
    }
}

fn wrap_in_const(
    kind: &DeriveKind,
    name: &syn::Ident,
    tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    println!("{}", tokens.to_string());

    let r#const = format_ident!(
        "_IMPL_{}_FOR_{}",
        match kind {
            DeriveKind::Serialize => "SERIALIZE",
            DeriveKind::Deserialize => "DESERIALIZE",
            DeriveKind::State => "STATE",
        },
        util::to_snake_case(&name.to_string()).to_uppercase()
    );

    quote! {
        const #r#const: () = {
            extern crate steit;

            use std::io::{self, Read};

            use steit::{
                de::Deserialize,
                iowrap,
                runtime::Runtime,
                ser::Serialize,
                // We don't import directly
                // to avoid confusing `serialize` and `deserialize` calls.
                varint,
            };

            #tokens
        };
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
