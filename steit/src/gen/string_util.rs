pub fn uncap_first_char(s: &str) -> String {
    let mut chars = s.chars();
    let mut out = String::new();

    if let Some(c) = chars.next() {
        out.extend(c.to_lowercase());
    }

    out.extend(chars);
    out
}

pub fn to_camel_case(s: &str, mut upper: bool) -> String {
    let mut chars = s.chars();
    let mut out = String::new();

    while let Some(c) = chars.next() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }

    out
}

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
