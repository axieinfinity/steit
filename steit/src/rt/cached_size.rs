use std::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

/// Cached object size to prevent duplicate size calculation in serialization.
///
/// It is always equal to itself so the containing object can use `#[derive(Eq)]`.
///
/// Reference: https://github.com/stepancheg/rust-protobuf/blob/68c7a5a/protobuf/src/cached_size.rs
#[derive(Default, Debug)]
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
    /// cached_size.set(1337);
    /// assert_eq!(cached_size.get(), 1337);
    /// ```
    #[inline]
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    /// Sets cached size.
    #[inline]
    pub fn set(&self, size: u32) {
        self.size.store(size, Ordering::Relaxed);
    }
}

impl Clone for CachedSize {
    fn clone(&self) -> CachedSize {
        CachedSize {
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

impl Hash for CachedSize {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // Ignore cached size in hash computation
    }
}
