use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{ctx::Context, derive, r#impl::Impl};

use super::DeriveSetting;

pub struct Union;

impl Union {
    pub fn parse(
        _setting: &DeriveSetting,
        context: &Context,
        _impl: &Impl<'_>,
        data: &mut syn::DataUnion,
    ) -> derive::Result<Self> {
        context.error(data.union_token, "cannot derive for unions yet");
        Err(())
    }
}

impl ToTokens for Union {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
