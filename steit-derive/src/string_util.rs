pub fn to_snake_case(s: impl AsRef<str>) -> String {
    let mut chars = s.as_ref().chars();
    let mut out = String::new();

    if let Some(c) = chars.next() {
        out.extend(c.to_lowercase());
    }

    for c in chars {
        if c.is_uppercase() {
            out.push('_');
        }

        out.extend(c.to_lowercase());
    }

    out
}
