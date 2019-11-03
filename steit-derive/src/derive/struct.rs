use std::collections::HashSet;

use crate::{
    attr::{Attr, AttrValue},
    context::Context,
    util,
};

use super::{
    derive::DeriveKind,
    field::{IndexedField, RuntimeField},
};

pub enum Derive<'a> {
    Serialize,
    Deserialize,
    State { runtime: RuntimeField<'a> },
}

pub struct Variant<'a> {
    variant: &'a syn::Variant,
    tag: u16,
}

impl<'a> Variant<'a> {
    pub fn ident(&self) -> &syn::Ident {
        &self.variant.ident
    }

    pub fn qual(&self) -> proc_macro2::TokenStream {
        let ident = self.ident();
        quote!(::#ident)
    }

    pub fn tag(&self) -> u16 {
        self.tag
    }
}

pub struct Struct<'a> {
    input: &'a syn::DeriveInput,
    variant: Option<Variant<'a>>,
    derive: Derive<'a>,
    indexed: Vec<IndexedField<'a>>,
}

macro_rules! map_fields {
    ($self:ident, $method:ident) => {
        $self.indexed.iter().map(|field| field.$method())
    };

    ($self:ident, $method:ident ($($rest:tt)*)) => {
        $self.indexed.iter().map(|field| field.$method($($rest)*))
    };
}

impl<'a> Struct<'a> {
    pub fn parse(
        context: &Context,
        kind: &DeriveKind,
        input: &'a syn::DeriveInput,
        object: impl quote::ToTokens,
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
        variant: Option<&'a syn::Variant>,
    ) -> Result<Self, ()> {
        let variant = if let Some(variant) = variant {
            Self::parse_attrs(context, &variant.ident, &variant.attrs).map(|tag| {
                Some(Variant {
                    variant,
                    tag: *tag.get(),
                })
            })
        } else {
            Ok(None)
        };

        let (runtimes, indexed): (Vec<_>, _) =
            fields
                .iter()
                .enumerate()
                .partition(|&(_index, field)| match type_name(&field.ty) {
                    Some(ident) if ident == "Runtime" => true,
                    _ => false,
                });

        if kind != &DeriveKind::State && runtimes.len() > 0 {
            context.error(
                fields,
                "unexpected `Runtime` field, as it's only allowed in #[derive(State)])",
            );
        };

        let derive = match kind {
            DeriveKind::Serialize => Derive::Serialize,
            DeriveKind::Deserialize => Derive::Deserialize,
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

                Derive::State { runtime }
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

    pub fn variant(&self) -> Option<&Variant> {
        self.variant.as_ref()
    }

    pub fn runtime(&self) -> Option<&RuntimeField<'_>> {
        if let Derive::State { runtime } = &self.derive {
            Some(runtime)
        } else {
            None
        }
    }

    pub fn indexed(&self) -> &Vec<IndexedField<'_>> {
        &self.indexed
    }

    pub fn get_ctor_and_setters(&self) -> Option<proc_macro2::TokenStream> {
        if let Derive::State { .. } = self.derive {
            let name = &self.input.ident;
            let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

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
            let qual = self.variant().map(|variant| variant.qual());
            let inits = self.get_inits();

            let setters: Vec<_> = self.get_setters();

            Some(quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #[doc = #doc]
                    pub fn #new(#(#args),*) -> Self {
                        #name #qual { #(#inits,)* }
                    }

                    #(#setters)*
                }
            })
        } else {
            None
        }
    }

    pub fn get_sizer_and_serializer(
        &self,
    ) -> Option<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
        match self.derive {
            Derive::State { .. } | Derive::Serialize => {
                let is_variant = self.variant.is_some();
                let sizers = map_fields!(self, get_sizer(is_variant));
                let serializers = map_fields!(self, get_serializer(is_variant));
                Some((quote!(#(#sizers)*), quote!(#(#serializers)*)))
            }

            Derive::Deserialize => None,
        }
    }

    pub fn get_deserializer(&self) -> Option<proc_macro2::TokenStream> {
        match self.derive {
            Derive::State { .. } | Derive::Deserialize => {
                let deserializers = map_fields!(self, get_deserializer(self.variant.is_some()));

                Some(quote! {
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
                                    let _: u8 = varint::deserialize(reader)?;
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
                })
            }

            Derive::Serialize => None,
        }
    }

    fn get_ctor_args(&self) -> Vec<proc_macro2::TokenStream> {
        if let Derive::State { runtime } = &self.derive {
            vec![runtime.get_arg()]
        } else {
            Vec::new()
        }
    }

    fn get_inits(&self) -> Vec<proc_macro2::TokenStream> {
        let mut inits: Vec<_> = map_fields!(self, get_init).collect();

        if let Derive::State { runtime } = &self.derive {
            inits.push(runtime.get_init(self.variant.as_ref().map(|variant| variant.tag)));
        }

        inits
    }

    fn get_setters(&self) -> Vec<proc_macro2::TokenStream> {
        if let Derive::State { .. } = &self.derive {
            map_fields!(self, get_setter(&self.input.ident, self.variant.as_ref())).collect()
        } else {
            Vec::new()
        }
    }
}

fn type_name(ty: &syn::Type) -> Option<&syn::Ident> {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            if let Some(segment) = path.segments.last() {
                Some(&segment.ident)
            } else {
                None
            }
        }

        _ => None,
    }
}
