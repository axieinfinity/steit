use std::collections::HashSet;

use crate::{
    context::Context,
    derivation::DerivationKind,
    field::{IndexedField, RuntimeField},
};

pub enum StructKind<'a> {
    Serialize,
    Deserialize,
    State { runtime: RuntimeField<'a> },
}

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
    variant: Option<&'a syn::Ident>,
    kind: StructKind<'a>,
    indexed: Vec<IndexedField<'a>>,
}

macro_rules! collect_fields {
    ($self:ident, $method:ident) => {
        $self.indexed.iter().map(|field| field.$method()).collect()
    };

    ($self:ident, $method:ident ($($rest:tt)*)) => {
        $self.indexed.iter().map(|field| field.$method($($rest)*)).collect()
    };
}

impl<'a> Struct<'a> {
    pub fn parse<O: quote::ToTokens>(
        context: &Context,
        kind: &DerivationKind,
        input: &'a syn::DeriveInput,
        object: O,
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        variant: Option<&'a syn::Ident>,
    ) -> Result<Self, ()> {
        let (runtimes, indexed): (Vec<_>, _) =
            fields.iter().enumerate().partition(|&(_index, field)| {
                match type_name(context, &field.ty) {
                    Some(ident) if ident == "Runtime" => true,
                    _ => false,
                }
            });

        if kind != &DerivationKind::State && runtimes.len() > 0 {
            context.error(
                fields,
                "unexpected `Runtime` field, as it's only allowed in #[derive(State)])",
            );
        };

        let struct_kind = match kind {
            DerivationKind::Serialize => Ok(StructKind::Serialize),
            DerivationKind::Deserialize => Ok(StructKind::Deserialize),
            DerivationKind::State => {
                if runtimes.len() == 0 {
                    context.error(object, "expected exactly one `Runtime` field, got none");
                    return Err(());
                }

                if runtimes.len() > 1 {
                    context.error(fields, "expected exactly one `Runtime` field, got multiple");
                }

                let runtime = runtimes
                    .first()
                    .map(|&(index, field)| RuntimeField::new(context, field, index))
                    .unwrap_or_else(|| unreachable!("expected a `Runtime` field"));

                Ok(StructKind::State { runtime })
            }
        };

        // Run it right away to accumulate all possible errors
        let indexed = Self::parse_indexed(context, kind, indexed);

        struct_kind.and_then(|kind| {
            indexed.map(|indexed| Self {
                input,
                variant,
                kind,
                indexed,
            })
        })
    }

    fn parse_indexed(
        context: &Context,
        kind: &DerivationKind,
        indexed: Vec<(usize, &'a syn::Field)>,
    ) -> Result<Vec<IndexedField<'a>>, ()> {
        let mut result = Vec::with_capacity(indexed.len());

        for (index, field) in &indexed {
            if let Ok(field) = IndexedField::parse(context, kind, field, *index) {
                result.push(field);
            }
        }

        if result.len() != indexed.len() {
            return Err(());
        }

        let mut tags = HashSet::new();
        let mut unique = true;

        for field in &result {
            let tag = field.tag();

            if !tags.insert(*tag.get()) {
                context.error(tag, "duplicate tag");
                unique = false;
            }
        }

        if unique {
            Ok(result)
        } else {
            Err(())
        }
    }

    fn get_ctor_args(&self) -> Vec<proc_macro2::TokenStream> {
        if let StructKind::State { runtime } = &self.kind {
            vec![runtime.to_arg()]
        } else {
            Vec::new()
        }
    }

    fn get_inits(&self) -> Vec<proc_macro2::TokenStream> {
        let mut inits: Vec<_> = collect_fields!(self, to_init);

        if let StructKind::State { runtime } = &self.kind {
            inits.push(runtime.to_init());
        }

        inits
    }

    fn get_setters(&self) -> Vec<proc_macro2::TokenStream> {
        if let StructKind::State { runtime } = &self.kind {
            collect_fields!(self, to_setter(runtime))
        } else {
            Vec::new()
        }
    }
}

impl<'a> quote::ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.input.ident;
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls = Vec::new();

        if let StructKind::State { .. } = self.kind {
            let (new, qual, doc) = match self.variant {
                Some(variant) => (
                    format_ident!("new_{}", to_snake_case(&variant.to_string())),
                    quote!(::#variant),
                    format!("Constructs a new `{}::{}`.", name, variant),
                ),
                None => (
                    format_ident!("new"),
                    quote!(),
                    format!("Constructs a new `{}`.", name),
                ),
            };

            let args = self.get_ctor_args();
            let inits = self.get_inits();

            let setters: Vec<_> = self.get_setters();

            impls.push(quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #[doc = #doc]
                    pub fn #new(#(#args),*) -> Self {
                        #name #qual { #(#inits,)* }
                    }

                    #(#setters)*
                }
            })
        }

        match self.kind {
            StructKind::State { .. } | StructKind::Serialize => {
                let sizers: Vec<_> = collect_fields!(self, to_sizer);
                let serializers: Vec<_> = collect_fields!(self, to_serializer);

                impls.push(quote! {
                    impl #impl_generics Serialize for #name #ty_generics #where_clause {
                        fn size(&self) -> u32 {
                            let mut size = 0;
                            #(#sizers)*
                            size
                        }

                        fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                            #(#serializers)*
                            Ok(())
                        }
                    }
                })
            }

            _ => (),
        }

        match self.kind {
            StructKind::State { .. } | StructKind::Deserialize => {
                let deserializers: Vec<_> = collect_fields!(self, to_deserializer);

                impls.push(quote! {
                    impl #impl_generics Deserialize for #name #ty_generics #where_clause {
                        fn deserialize<R: io::Read>(&mut self, reader: &mut R) -> io::Result<()> {
                            let reader = &mut iowrap::Eof::new(reader);

                            while !reader.eof()? {
                                let key: u32 = varint::deserialize(reader)?;
                                let tag = (key >> 3) as u16;
                                let wire_type = (key & 7) as u8;

                                match tag {
                                    #(#deserializers,)*

                                    _ => match wire_type {
                                        0 => {
                                            // Other than `u8`, any int type will do
                                            // since we deserialize just to ignore the whole varint.
                                            <u8 as varint::Varint>::deserialize(reader)?;
                                        }

                                        2 => {
                                            let size = varint::deserialize(reader)?;
                                            let mut buf = Vec::new();

                                            reader
                                                .by_ref()
                                                .take(size)
                                                .read_to_end(&mut buf)?;
                                        }

                                        _ => {
                                            return Err(io::Error::new(
                                                io::ErrorKind::InvalidData,
                                                format!("unexpected wire type {}", wire_type),
                                            ));
                                        }
                                    },
                                }
                            }

                            Ok(())
                        }
                    }
                });
            }

            _ => (),
        }

        tokens.extend(with_preimports(&self.kind, name, quote!(#(#impls)*)));
    }
}

fn type_name<'a>(context: &Context, ty: &'a syn::Type) -> Option<&'a syn::Ident> {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            if let Some(segment) = path.segments.last() {
                Some(&segment.ident)
            } else {
                context.error(ty, "expected a non-empty type path");
                None
            }
        }

        _ => {
            context.error(ty, "expected a type path");
            None
        }
    }
}

fn with_preimports(
    kind: &StructKind,
    name: &syn::Ident,
    tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let r#const = format_ident!(
        "_IMPL_{}_FOR_{}",
        match kind {
            StructKind::Serialize => "SERIALIZE",
            StructKind::Deserialize => "DESERIALIZE",
            StructKind::State { .. } => "STATE",
        },
        to_snake_case(&name.to_string()).to_uppercase()
    );

    quote! {
        const #r#const: () = {
            extern crate steit;

            use std::io::{self, Read};

            use steit::{
                de::Deserialize,
                iowrap,
                ser::Serialize,
                // We don't import directly
                // to avoid confusing `serialize` and `deserialize` calls.
                varint,
            };

            #tokens
        };
    }
}

fn to_snake_case(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut out = String::new();

    while let Some(c) = chars.next() {
        out.extend(c.to_lowercase());

        if let Some(next_c) = chars.peek() {
            if next_c.is_uppercase() {
                out.push('_');
            }
        }
    }

    out
}
