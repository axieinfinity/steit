use proc_macro2::TokenStream;

pub struct Impl {
    ident: syn::Ident,
    generics: syn::Generics,
}

impl Impl {
    pub fn new(input: &syn::DeriveInput) -> Self {
        Self {
            ident: input.ident.clone(),
            generics: input.generics.clone(),
        }
    }

    pub fn impl_for<'a>(
        &self,
        r#trait: impl Into<Option<&'a str>>,
        tokens: TokenStream,
    ) -> TokenStream {
        let name = &self.ident;
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
