use crate::context::Context;

pub struct AttrHolder<'a, T> {
    context: &'a Context,
    name: &'static str,
    value: Option<T>,
    tokens: proc_macro2::TokenStream,
}

impl<'a, T> AttrHolder<'a, T> {
    pub fn new(context: &'a Context, name: &'static str) -> Self {
        Self {
            context,
            name,
            value: None,
            tokens: proc_macro2::TokenStream::new(),
        }
    }

    pub fn set<O: quote::ToTokens>(&mut self, object: O, value: T) {
        let tokens = object.into_token_stream();

        if self.value.is_some() {
            self.context
                .error(tokens, format!("duplicate state attribute `{}`", self.name));
        } else {
            self.value = Some(value);
            self.tokens = tokens;
        }
    }

    pub fn attr(self) -> Option<Attr<T>> {
        let AttrHolder { value, tokens, .. } = self;
        value.map(|value| Attr { value, tokens })
    }
}

pub struct Attr<T> {
    value: T,
    tokens: proc_macro2::TokenStream,
}

impl<T> Attr<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> quote::ToTokens for Attr<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}
