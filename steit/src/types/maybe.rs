use crate::steit_derive;

use crate::{
    rt::{Runtime, SizeCache},
    state::State,
};

#[steit_derive(Clone, PartialEq, Eq, Hash, Debug, State)]
#[steit(steit_owned)]
pub enum Maybe<T: State> {
    #[steit(tag = 0)]
    #[steit(no_size_cache)]
    None,
    #[steit(tag = 1)]
    Some {
        #[steit(tag = 0)]
        value: T,
    },
}

impl<T: State> Maybe<T> {
    #[inline]
    pub fn some(runtime: Runtime, value: T) -> Self {
        Maybe::Some {
            value,
            size_cache: SizeCache::new(),
            runtime,
        }
    }

    #[inline]
    pub fn none(runtime: Runtime) -> Self {
        Maybe::None { runtime }
    }

    pub fn from_option(runtime: Runtime, option: Option<T>) -> Self {
        match option {
            Some(value) => Self::some(runtime, value),
            None => Self::none(runtime),
        }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        matches!(self, Maybe::Some { .. })
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

impl<T: PartialEq + State> PartialEq<Option<T>> for Maybe<T> {
    fn eq(&self, other: &Option<T>) -> bool {
        match self {
            Maybe::Some { value, .. } => match other {
                Some(other_value) => other_value == value,
                None => false,
            },

            Maybe::None { .. } => other.is_none(),
        }
    }
}
