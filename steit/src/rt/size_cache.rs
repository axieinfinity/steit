use std::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

/// Caches serialization size to prevent duplicate calculation.
///
/// A [`SizeCache`] is always equal to itself so its containing object can use `#[derive(Eq)]`.
///
/// This references [`CachedSize`] from [rust-protobuf].
///
/// [`SizeCache`]: struct.SizeCache.html
/// [rust-protobuf]: https://github.com/stepancheg/rust-protobuf
/// [`CachedSize`]: https://github.com/stepancheg/rust-protobuf/blob/68c7a5a/protobuf/src/cached_size.rs
#[derive(Default, Debug)]
pub struct SizeCache {
    size: AtomicU32,
}

impl SizeCache {
    /// Creates a new [`SizeCache`] and initializes it to 0.
    ///
    /// [`SizeCache`]: struct.SizeCache.html
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets cached size.
    ///
    /// ```
    /// # use steit::SizeCache;
    /// let size_cache = SizeCache::new();
    /// assert_eq!(size_cache.get(), 0);
    /// ```
    #[inline]
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed)
    }

    /// Sets cached size.
    ///
    /// ```
    /// # use steit::SizeCache;
    /// let size_cache = SizeCache::new();
    /// size_cache.set(1337);
    /// assert_eq!(size_cache.get(), 1337);
    /// ```
    #[inline]
    pub fn set(&self, size: u32) {
        self.size.store(size, Ordering::Relaxed);
    }
}

impl Clone for SizeCache {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            size: AtomicU32::new(self.get()),
        }
    }
}

impl PartialEq for SizeCache {
    #[inline]
    fn eq(&self, _other: &SizeCache) -> bool {
        true
    }
}

impl Eq for SizeCache {}

impl Hash for SizeCache {
    #[inline]
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // Ignore cached size in hash computation
    }
}

#[cfg(test)]
mod tests {
    use crate::test_case;

    use super::SizeCache;

    fn assert_back_and_forth(value: u32) {
        let size_cache = SizeCache::new();
        size_cache.set(value);
        assert_eq!(size_cache.get(), value);
    }

    test_case!(back_and_forth_01: assert_back_and_forth; 0);
    test_case!(back_and_forth_02: assert_back_and_forth; 1);
    test_case!(back_and_forth_03: assert_back_and_forth; 1337);
    test_case!(back_and_forth_04: assert_back_and_forth; 1_000_000_007);
}
