use proc_macro2::TokenStream;

pub struct Impler<'a> {
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
}

impl<'a> Impler<'a> {
    pub fn new(name: &'a syn::Ident, generics: &'a syn::Generics) -> Self {
        Self { name, generics }
    }

    pub fn name(&self) -> &syn::Ident {
        self.name
    }

    pub fn r#impl(&self, tokens: TokenStream) -> TokenStream {
        self.impl_for(None, tokens)
    }

    pub fn impl_with(&self, bounds: &[&str], tokens: TokenStream) -> TokenStream {
        self.impl_for_with(None, bounds, tokens)
    }

    pub fn impl_for_with<'b>(
        &self,
        r#trait: impl Into<Option<&'b str>>,
        bounds: &[&str],
        tokens: TokenStream,
    ) -> TokenStream {
        let r#trait = r#trait.into();
        let mut generics = self.generics.clone();

        for type_param in &mut generics.type_params_mut() {
            for bound in bounds {
                type_param.bounds.push(syn::parse_str(bound).unwrap());
            }
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let r#for = r#trait.map(|r#trait| {
            let r#trait = format_ident!("{}", r#trait);
            quote!(#r#trait for)
        });

        let name = &self.name;

        quote! {
            impl #impl_generics #r#for #name #ty_generics #where_clause {
                #tokens
            }
        }
    }

    pub fn impl_for<'b>(
        &self,
        r#trait: impl Into<Option<&'b str>>,
        tokens: TokenStream,
    ) -> TokenStream {
        let r#trait = r#trait.into();
        let bounds: Vec<_> = r#trait.iter().copied().collect();
        self.impl_for_with(r#trait, &bounds, tokens)
    }
}
