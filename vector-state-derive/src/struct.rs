use std::collections::HashSet;

use crate::{
    context::Context,
    field::{IndexedField, PathField},
};

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
    variant: Option<&'a syn::Ident>,
    path: PathField<'a>,
    indexed: Vec<IndexedField<'a>>,
}

impl<'a> Struct<'a> {
    pub fn parse<O: quote::ToTokens>(
        context: &Context,
        input: &'a syn::DeriveInput,
        object: O,
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        variant: Option<&'a syn::Ident>,
    ) -> Result<Self, ()> {
        let (paths, indexed): (Vec<_>, _) =
            fields.iter().enumerate().partition(|&(_index, field)| {
                match type_name(context, &field.ty) {
                    Some(ident) if ident == "Path" => true,
                    _ => false,
                }
            });

        if paths.len() == 0 {
            context.error(object, "expected exactly one `Path` field, got none");
            return Err(());
        }

        if paths.len() > 1 {
            context.error(fields, "expected exactly one `Path` field, got multiple");
            return Err(());
        }

        let path = paths
            .first()
            .map(|&(index, field)| PathField::new(field, index))
            .unwrap_or_else(|| unreachable!("expected a `Path` field"));

        Self::parse_indexed(context, indexed).map(|indexed| Self {
            input,
            variant,
            path,
            indexed,
        })
    }

    fn parse_indexed(
        context: &Context,
        indexed: Vec<(usize, &'a syn::Field)>,
    ) -> Result<Vec<IndexedField<'a>>, ()> {
        let mut result = Vec::with_capacity(indexed.len());

        for (index, field) in &indexed {
            if let Ok(field) = IndexedField::parse(context, field, *index) {
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

    fn get_inits(&self) -> Vec<proc_macro2::TokenStream> {
        let mut inits: Vec<_> = self.indexed.iter().map(|field| field.to_init()).collect();
        inits.push(self.path.to_init());
        inits
    }
}

macro_rules! collect_fields {
    ($self:ident, $method:ident) => {
        $self.indexed.iter().map(|field| field.$method()).collect()
    };

    ($self:ident, $method:ident ($($rest:tt)*)) => {
        $self.indexed.iter().map(|field| field.$method($($rest)*)).collect()
    };
}

impl<'a> quote::ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.input.ident;
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

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

        let arg = self.path.to_arg();
        let inits = self.get_inits();

        let setters: Vec<_> = collect_fields!(self, to_setter(&self.path));

        let sizers: Vec<_> = collect_fields!(self, to_sizer);
        let serializers: Vec<_> = collect_fields!(self, to_serializer);
        let deserializers: Vec<_> = collect_fields!(self, to_deserializer);

        let impls = with_preimports(
            name,
            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #[doc = #doc]
                    pub fn #new(#arg) -> Self {
                        #name #qual { #(#inits,)* }
                    }

                    #(#setters)*
                }

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

                impl #impl_generics Deserialize for #name #ty_generics #where_clause {
                    fn deserialize<R: io::Read>(&mut self, reader: &mut R) -> io::Result<()> {
                        let reader = &mut iowrap::Eof::new(reader);

                        while !reader.eof()? {
                            let key: u32 = varint::Varint::deserialize(reader)?;
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
                                        let size = varint::Varint::deserialize(reader)?;
                                        let mut buf = Vec::new();

                                        reader
                                            .by_ref()
                                            .take(size)
                                            .read_to_end(&mut buf)?;
                                    }

                                    _ => {
                                        Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            format!("unexpected wire type {}", wire_type),
                                        ))?;
                                    }
                                },
                            }
                        }

                        Ok(())
                    }
                }

            },
        );

        tokens.extend(impls);
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
    name: &syn::Ident,
    tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let r#const = format_ident!(
        "_IMPL_STATE_FOR_{}",
        to_snake_case(&name.to_string()).to_uppercase()
    );

    quote! {
        const #r#const: () = {
            extern crate vector_state;

            use std::io::{self, Read};

            use vector_state::{
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
