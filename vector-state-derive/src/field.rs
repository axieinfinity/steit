pub enum FieldExt<'a> {
    Path(PathExt),
    Indexed(IndexedFieldExt<'a>),
}

impl<'a> FieldExt<'a> {
    pub fn new(field: &'a syn::Field, field_index: usize, named: bool) -> Self {
        let ty = &field.ty;
        let is_path = ty_name(&field.ty) == "Path";

        let ident = if named {
            field.ident.clone().unwrap()
        } else {
            format_ident!("f_{}", field_index)
        };

        if is_path {
            FieldExt::Path(PathExt { ident, named })
        } else {
            let full_ty_name = quote!(#ty).to_string();
            let is_state = full_ty_name != "i32" && full_ty_name != "bool";
            let (index, default) = Self::parse_attrs(&field.attrs, is_state);

            FieldExt::Indexed(IndexedFieldExt {
                ty,
                ident,
                named,
                index,
                default,
                is_state,
            })
        }
    }

    fn parse_attrs(
        attrs: &[syn::Attribute],
        is_state: bool,
    ) -> (u16, Option<proc_macro2::TokenStream>) {
        let mut tag = None;
        let mut default = None;

        for attr in attrs {
            if let Ok(syn::Meta::List(syn::MetaList { path, nested, .. })) = attr.parse_meta() {
                if !path.is_ident("state") {
                    continue;
                }

                // Now we have already have #[state(nested_1, nested_2, ...)].
                // We will try to parse each nested meta.

                for meta in nested {
                    // We only care about name-value metas, such as "tag = 1".
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
                                if is_state {
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

pub struct PathExt {
    ident: syn::Ident,
    named: bool,
}

impl PathExt {
    pub fn as_arg(&self) -> proc_macro2::TokenStream {
        let field_name = &self.ident;
        quote!(#field_name: Path)
    }

    pub fn as_init(&self) -> proc_macro2::TokenStream {
        let field_name = &self.ident;
        let init = quote!(#field_name);

        if self.named {
            quote!(#field_name: #init)
        } else {
            quote!(#init)
        }
    }
}

pub struct IndexedFieldExt<'a> {
    ty: &'a syn::Type,
    ident: syn::Ident,
    named: bool,
    index: u16,
    default: Option<proc_macro2::TokenStream>,
    is_state: bool,
}

impl<'a> IndexedFieldExt<'a> {
    pub fn as_init(&self, path: &PathExt) -> proc_macro2::TokenStream {
        let field_name = &self.ident;
        let path = &path.ident;

        let init = if !self.is_state {
            match self.default {
                Some(ref default) => quote!(#default),
                None => quote!(Default::default()),
            }
        } else {
            let ty_name = ty_name(self.ty);
            let index = self.index;
            quote!(#ty_name::new(#path.derive(#index)))
        };

        if self.named {
            quote!(#field_name: #init)
        } else {
            quote!(#init)
        }
    }
}

fn ty_name(ty: &syn::Type) -> &syn::Ident {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            &path
                .segments
                .last()
                .expect("expected at least one path segment")
                .ident
        }
        _ => panic!("expected a path type"),
    }
}
