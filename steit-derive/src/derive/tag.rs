/// Maximum possible tag.
pub const TAG_MAX: u32 = 0x1fffffff;

pub fn validate(tag: u32) -> Result<u32, &'static str> {
    if tag > TAG_MAX {
        return Err("tag must not be greater than 2^29 - 1");
    }

    Ok(tag)
}
