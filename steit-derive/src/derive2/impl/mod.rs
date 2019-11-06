use super::{ctx::Context, Derive};

pub mod r#enum;
pub mod union;
pub mod variant;

pub struct ImplInput<'a> {
    derive: &'a Derive,
    context: &'a Context,
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
}

impl<'a> ImplInput<'a> {
    pub fn new(
        derive: &'a Derive,
        context: &'a Context,
        ident: &'a syn::Ident,
        generics: &'a syn::Generics,
    ) -> Self {
        Self {
            derive,
            context,
            ident,
            generics,
        }
    }
}
