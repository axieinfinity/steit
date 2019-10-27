pub fn type_name(ty: &syn::Type) -> &syn::Ident {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            &path
                .segments
                .last()
                .expect("expected at least one type path segment")
                .ident
        }

        _ => panic!("expected a type path"),
    }
}

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
