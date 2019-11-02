use std::collections::HashSet;

use crate::{
    attr::{Attr, AttrValue},
    context::Context,
    derive::DeriveKind,
    field::{IndexedField, RuntimeField},
    util,
};

pub enum StructDerive<'a> {
    Serialize,
    Deserialize,
    State { runtime: RuntimeField<'a> },
}

pub struct StructVariant<'a> {
    variant: &'a syn::Variant,
    tag: u16,
}

impl<'a> StructVariant<'a> {
    pub fn ident(&self) -> &syn::Ident {
        &self.variant.ident
    }

    pub fn tag(&self) -> u16 {
        self.tag
    }
}

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
    variant: Option<StructVariant<'a>>,
    derive: StructDerive<'a>,
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
        kind: &DeriveKind,
        input: &'a syn::DeriveInput,
        object: O,
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        variant: Option<&'a syn::Variant>,
    ) -> Result<Self, ()> {
        let variant = if let Some(variant) = variant {
            Self::parse_attrs(context, &variant.ident, &variant.attrs).map(|tag| {
                Some(StructVariant {
                    variant,
                    tag: *tag.get(),
                })
            })
        } else {
            Ok(None)
        };

        let (runtimes, indexed): (Vec<_>, _) =
            fields.iter().enumerate().partition(|&(_index, field)| {
                match type_name(context, &field.ty) {
                    Some(ident) if ident == "Runtime" => true,
                    _ => false,
                }
            });

        if kind != &DeriveKind::State && runtimes.len() > 0 {
            context.error(
                fields,
                "unexpected `Runtime` field, as it's only allowed in #[derive(State)])",
            );
        };

        let derive = match kind {
            DeriveKind::Serialize => StructDerive::Serialize,
            DeriveKind::Deserialize => StructDerive::Deserialize,
            DeriveKind::State => {
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

                StructDerive::State { runtime }
            }
        };

        Self::parse_indexed(context, kind, indexed).and_then(|indexed| {
            variant.map(|variant| Self {
                input,
                variant,
                derive,
                indexed,
            })
        })
    }

    fn parse_attrs(
        context: &Context,
        variant: &syn::Ident,
        attrs: &[syn::Attribute],
    ) -> Result<(AttrValue<u16>), ()> {
        let mut tag_attr = Attr::new(context, "tag");
        let mut tag_encountered = false;

        for item in attrs
            .iter()
            .flat_map(|attr| util::get_steit_meta_items(context, attr))
            .flatten()
        {
            match &item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(item)) if item.path.is_ident("tag") => {
                    tag_encountered = true;

                    if let Ok(lit) = util::get_lit_int(context, "tag", &item.lit) {
                        if let Ok(tag) = lit.base10_parse() {
                            tag_attr.set(lit, tag);
                        } else {
                            context.error(lit, format!("unable to parse #[steit(tag = {})]", lit));
                        }
                    }
                }

                syn::NestedMeta::Meta(item) => {
                    let path = item.path();
                    let path = quote!(#path).to_string().replace(' ', "");
                    context.error(item.path(), format!("unknown steit attribute `{}`", path));
                }

                syn::NestedMeta::Lit(lit) => {
                    context.error(lit, "unexpected literal in steit attributes");
                }
            }
        }

        if let Some(tag) = tag_attr.value() {
            Ok(tag)
        } else {
            if !tag_encountered {
                context.error(variant, "expected a `tag` attribute #[steit(tag = ...)]");
            }

            Err(())
        }
    }

    fn parse_indexed(
        context: &Context,
        kind: &DeriveKind,
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

    pub fn qual(&self) -> Option<proc_macro2::TokenStream> {
        self.variant.as_ref().map(|variant| {
            let variant = variant.ident();
            quote!(::#variant)
        })
    }

    pub fn runtime(&self) -> Option<proc_macro2::TokenStream> {
        if let StructDerive::State { runtime } = &self.derive {
            Some(runtime.get_access())
        } else {
            None
        }
    }

    fn get_ctor_args(&self) -> Vec<proc_macro2::TokenStream> {
        if let StructDerive::State { runtime } = &self.derive {
            vec![runtime.to_arg()]
        } else {
            Vec::new()
        }
    }

    fn get_inits(&self) -> Vec<proc_macro2::TokenStream> {
        let mut inits: Vec<_> = collect_fields!(self, to_init);

        if let StructDerive::State { runtime } = &self.derive {
            inits.push(runtime.to_init(self.variant.as_ref().map(|variant| variant.tag)));
        }

        inits
    }

    fn get_setters(&self) -> Vec<proc_macro2::TokenStream> {
        if let StructDerive::State { .. } = &self.derive {
            let variant = self.variant.as_ref();
            collect_fields!(self, to_setter(&self.input.ident, variant))
        } else {
            Vec::new()
        }
    }
}

impl<'a> quote::ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.input.ident;
        let qual = self.qual();
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

        let mut impls = Vec::new();

        if let StructDerive::State { .. } = self.derive {
            let (new, doc) = match &self.variant {
                Some(variant) => (
                    format_ident!("new_{}", util::to_snake_case(&variant.ident().to_string())),
                    format!("Constructs a new `{}::{}`.", name, variant.ident()),
                ),
                None => (
                    format_ident!("new"),
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

        match self.derive {
            StructDerive::State { .. } | StructDerive::Serialize => {
                let sizers: Vec<_> = collect_fields!(self, to_sizer);
                let serializers: Vec<_> = collect_fields!(self, to_serializer);

                impls.push(quote! {
                    /* impl #impl_generics Serialize for #name #ty_generics #where_clause {
                        fn size(&self) -> u32 {
                            let mut size = 0;
                            #(#sizers)*
                            size
                        }

                        fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                            self.size().serialize(writer)?;
                            #(#serializers)*
                            Ok(())
                        }
                    } */
                })
            }

            _ => (),
        }

        match self.derive {
            StructDerive::State { .. } | StructDerive::Deserialize => {
                let deserializers: Vec<_> = collect_fields!(self, to_deserializer);

                impls.push(quote! {
                    /* impl #impl_generics Deserialize for #name #ty_generics #where_clause {
                        fn deserialize<R: io::Read>(&mut self, reader: &mut R) -> io::Result<()> {
                            let size = varint::deserialize(reader)?;
                            let reader = &mut iowrap::Eof::new(reader.by_ref().take(size));

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
                    } */
                });
            }

            _ => (),
        }

        tokens.extend(quote!(#(#impls)*));
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
