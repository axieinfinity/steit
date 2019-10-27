extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod field;

use field::FieldExt;

#[proc_macro_derive(State, attributes(state))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let output = match input.data {
        syn::Data::Enum(ref data) => state_for_enum(&input, data),
        syn::Data::Struct(ref data) => state_for_struct(&input, &data.fields, None),
        syn::Data::Union(_) => unimplemented!("doesn't work with unions yet"),
    };

    output.into()
}

fn state_for_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    if data.variants.is_empty() {
        panic!("cannot derive State for enums with zero variants");
    }

    let impls = data.variants.iter().map(|variant| {
        if variant.discriminant.is_some() {
            panic!("cannot derive State for enums with discriminants");
        }

        state_for_struct(input, &variant.fields, Some(&variant.ident))
    });

    quote!(#(#impls)*)
}

fn state_for_struct(
    input: &syn::DeriveInput,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => state_impl(&input, Some(&fields.named), true, variant),
        syn::Fields::Unnamed(ref fields) => {
            state_impl(&input, Some(&fields.unnamed), false, variant)
        }
        syn::Fields::Unit => state_impl(&input, None, false, variant),
    }
}

fn state_impl(
    input: &syn::DeriveInput,
    fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
    named: bool,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (new, qual, doc) = match variant {
        None => (
            format_ident!("new"),
            quote!(),
            format!("Constructs a new `{}`.", name),
        ),
        Some(ref variant) => (
            format_ident!("new_{}", to_snake_case(&variant.to_string())),
            quote!(::#variant),
            format!("Constructs a new `{}::{}`.", name, variant),
        ),
    };

    let mut fields: Vec<_> = fields
        .expect("expected at least a path field")
        .iter()
        .enumerate()
        .map(|(index, field)| FieldExt::new(field, index, named))
        .collect();

    let path = fields
        .iter()
        .find_map(|field| match field {
            FieldExt::Path(path) => Some(path),
            FieldExt::Indexed(_) => None,
        })
        .expect("expected a path field");

    let arg = path.as_arg();

    let fields = fields.iter().filter_map(|f| match f {
        FieldExt::Indexed(field) => Some(field),
        FieldExt::Path(_) => None,
    });

    let mut inits: Vec<_> = fields.map(|field| field.as_init(&path)).collect();
    inits.push(path.as_init());

    let inits = if named {
        quote![{ #(#inits),* }]
    } else {
        quote![( #(#inits),* )]
    };

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new(#arg) -> Self {
                #name #qual #inits
            }
        }
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
