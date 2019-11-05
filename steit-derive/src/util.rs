use crate::context::Context;

pub fn to_snake_case(s: &str) -> String {
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

pub fn get_steit_meta_items(
    context: &Context,
    attr: &syn::Attribute,
) -> Result<Vec<syn::NestedMeta>, ()> {
    if !attr.path.is_ident("steit") {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(syn::Meta::List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(other) => {
            context.error(other, "expected #[steit(...)]");
            Err(())
        }
        Err(error) => {
            context.syn_error(error);
            Err(())
        }
    }
}

pub fn get_lit_str<'a>(
    context: &Context,
    name: &'static str,
    lit: &'a syn::Lit,
) -> Result<&'a syn::LitStr, ()> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit)
    } else {
        context.error(
            lit,
            format!(
                "expected `{}` attribute to be represented by a string",
                name
            ),
        );

        Err(())
    }
}

pub fn get_lit_int<'a>(
    context: &Context,
    name: &'static str,
    lit: &'a syn::Lit,
) -> Result<&'a syn::LitInt, ()> {
    if let syn::Lit::Int(lit) = lit {
        Ok(lit)
    } else {
        context.error(
            lit,
            format!("expected `{}` attribute to be represented by an int", name),
        );

        Err(())
    }
}
