extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

#[proc_macro_derive(State, attributes(state))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("couldn't parse item");

    match ast.data {
        syn::Data::Enum(ref _e) => panic!("doesn't work with enums yet"),
        syn::Data::Struct(ref s) => new_for_struct(&ast, &s.fields, None).into(),
        syn::Data::Union(ref _u) => panic!("doesn't work with unions yet"),
    }
}

fn new_for_struct(
    ast: &syn::DeriveInput,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => new_impl(&ast, Some(&fields.named), true, variant),
        syn::Fields::Unit => new_impl(&ast, None, false, variant),
        syn::Fields::Unnamed(ref fields) => new_impl(&ast, Some(&fields.unnamed), false, variant),
    }
}

fn new_impl(
    ast: &syn::DeriveInput,
    _fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
    _named: bool,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (new, qual, doc) = match variant {
        None => (
            syn::Ident::new("new", proc_macro2::Span::call_site()),
            quote!(),
            format!("Constructs a new `{}`.", name),
        ),
        Some(ref variant) => (
            syn::Ident::new(
                &format!("new_{}", to_snake_case(&variant.to_string())),
                proc_macro2::Span::call_site(),
            ),
            quote!(::#variant),
            format!("Constructs a new `{}::{}`.", name, variant),
        ),
    };

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new() -> Self {
                #name #qual {}
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
