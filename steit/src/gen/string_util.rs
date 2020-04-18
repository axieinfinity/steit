pub fn uncap_first_char(s: impl AsRef<str>) -> String {
    let mut chars = s.as_ref().chars();
    let mut out = String::new();

    if let Some(c) = chars.next() {
        out.extend(c.to_lowercase());
    }

    out.extend(chars);
    out
}

pub fn to_camel_case(s: impl AsRef<str>, mut upper: bool) -> String {
    let mut out = String::new();

    for c in s.as_ref().chars() {
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
