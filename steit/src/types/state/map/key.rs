pub trait MapKey: Sized {
    fn as_tag(&self) -> u16;
    fn try_from_tag<'a>(tag: u16) -> Result<Self, &'a str>;
}

impl MapKey for u16 {
    #[inline]
    fn as_tag(&self) -> u16 {
        *self
    }

    #[inline]
    fn try_from_tag<'a>(tag: u16) -> Result<Self, &'a str> {
        Ok(tag)
    }
}
