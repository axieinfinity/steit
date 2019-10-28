// Note that we intentionally exclude some unsupported primitive types
const PRIMITIVE_TYPES: &[&str] = &["bool", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"];

pub enum FieldKind {
    Primitive {
        default: Option<proc_macro2::TokenStream>,
    },

    State,
}

pub struct IndexedField<'a> {
    name: Option<syn::Ident>,
    ty: &'a syn::Type,
    index: usize,
    tag: u16,
    kind: FieldKind,
}

impl<'a> IndexedField<'a> {
    pub fn new(field: &'a syn::Field, index: usize) -> Self {
        let ty = &field.ty;

        let full_type_name = quote!(#ty).to_string();
        let is_primitive = PRIMITIVE_TYPES.contains(&&*full_type_name);

        let (tag, default) = Self::parse_attrs(&field.attrs, is_primitive);

        let kind = if is_primitive {
            FieldKind::Primitive { default }
        } else {
            FieldKind::State
        };

        Self {
            name: field.ident.clone(),
            ty,
            index,
            tag,
            kind,
        }
    }

    pub fn debug(&self) {
        println!("{:?}, {}, {}", self.name, self.index, self.tag)
    }

    fn parse_attrs(
        attrs: &[syn::Attribute],
        is_primitive: bool,
    ) -> (u16, Option<proc_macro2::TokenStream>) {
        let mut tag = None;
        let mut default = None;

        for attr in attrs {
            if let Ok(syn::Meta::List(syn::MetaList { path, nested, .. })) = attr.parse_meta() {
                if !path.is_ident("state") {
                    continue;
                }

                // Now we have already have #[state(nested_1, nested_2, ...)].
                // We will try to parse each nested meta item.

                for meta in nested {
                    // We only care about name-value meta items, such as "tag = 1".
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(ref name_value)) = meta {
                        match name_value {
                            // tag = {int}
                            syn::MetaNameValue {
                                path,
                                lit: syn::Lit::Int(ref lit),
                                ..
                            } if path.is_ident("tag") => {
                                if tag.is_some() {
                                    panic!("expected only one tag number, encountered many");
                                }

                                tag = Some(lit.base10_parse::<u16>().unwrap())
                            }

                            // default = {string}
                            syn::MetaNameValue {
                                path,
                                lit: syn::Lit::Str(ref lit),
                                ..
                            } if path.is_ident("default") => {
                                if !is_primitive {
                                    panic!("a state field cannot have a default value")
                                }

                                if default.is_some() {
                                    panic!("expected only one default value, encountered many");
                                }

                                default = Some(lit.value().parse().expect(&format!(
                                    "invalid expression in #[state(default = \"{}\")]",
                                    lit.value()
                                )));
                            }

                            _ => (),
                        }
                    }
                }
            }
        }

        (tag.expect("expected a tag number"), default)
    }
}

pub struct PathField {}
