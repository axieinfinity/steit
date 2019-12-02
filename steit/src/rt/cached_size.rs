use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

/// Cached size to prevent duplicate calculation in serialization.
///
/// It is always equal to itself so the containing object can use `#[derive(Eq)]`.
///
/// Reference: https://github.com/stepancheg/rust-protobuf/blob/68c7a5a/protobuf/src/cached_size.rs
#[derive(Default)]
pub struct CachedSize {
    size: AtomicU32,
}

impl CachedSize {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets cached size.
    ///
    /// ```
    /// use steit::CachedSize;
    ///
    /// let cached_size = CachedSize::new();
    /// assert_eq!(cached_size.get(), 0);
    /// ```
    #[inline]
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    /// Sets cached size.
    ///
    /// ```
    /// use steit::CachedSize;
    ///
    /// let cached_size = CachedSize::new();
    /// cached_size.set(1337);
    /// assert_eq!(cached_size.get(), 1337);
    /// ```
    #[inline]
    pub fn set(&self, size: u32) {
        self.size.store(size, Ordering::Relaxed);
    }
}

impl Clone for CachedSize {
    fn clone(&self) -> Self {
        Self {
            size: AtomicU32::new(self.get()),
        }
    }
}

impl PartialEq for CachedSize {
    fn eq(&self, _other: &CachedSize) -> bool {
        true
    }
}

impl Eq for CachedSize {}

impl fmt::Debug for CachedSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl Hash for CachedSize {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // Ignore cached size in hash computation
    }
}

#[cfg(test)]
mod tests {
    use crate::test_case;

    use super::CachedSize;

    fn assert_back_and_forth(value: u32) {
        let cached_size = CachedSize::new();
        cached_size.set(value);
        assert_eq!(cached_size.get(), value);
    }

    test_case!(back_and_forth_01: assert_back_and_forth; 0);
    test_case!(back_and_forth_02: assert_back_and_forth; 1);
    test_case!(back_and_forth_03: assert_back_and_forth; 1337);
    test_case!(back_and_forth_04: assert_back_and_forth; 1_000_000_007);
}
