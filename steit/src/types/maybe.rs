use crate::steit_derive;

#[steit_derive(Clone, PartialEq, Eq, Hash, Debug, State)]
#[steit(steit_owned)]
pub enum Maybe<T> {
    #[steit(tag = 0)]
    None,
    #[steit(tag = 1)]
    Some {
        #[steit(tag = 0)]
        value: T,
    },
}

impl<T> Maybe<T> {
    #[inline]
    pub fn is_some(&self) -> bool {
        matches!(self, Maybe::Some { .. })
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
