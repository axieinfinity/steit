pub fn to_snake_case(s: &str) -> String {
    let mut chars = s.chars();
    let mut out = String::new();

    if let Some(c) = chars.next() {
        out.extend(c.to_lowercase());
    }

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            out.push('_');
        }

        out.extend(c.to_lowercase());
    }

    out
}
