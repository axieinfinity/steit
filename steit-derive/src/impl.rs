use proc_macro2::TokenStream;

pub struct Impl<'a> {
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
}

impl<'a> Impl<'a> {
    pub fn new(name: &'a syn::Ident, generics: &'a syn::Generics) -> Self {
        Self { name, generics }
    }

    pub fn name(&self) -> &syn::Ident {
        self.name
    }

    pub fn impl_for<'b>(
        &self,
        r#trait: impl Into<Option<&'b str>>,
        tokens: TokenStream,
    ) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let r#for = r#trait.into().map(|r#trait| {
            let r#trait = format_ident!("{}", r#trait);
            quote!(#r#trait for)
        });

        quote! {
            impl #impl_generics #r#for #name #ty_generics #where_clause {
                #tokens
            }
        }
    }

    pub fn r#impl(&self, tokens: TokenStream) -> TokenStream {
        self.impl_for(None, tokens)
    }
}
