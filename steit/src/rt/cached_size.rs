use std::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

// Reference: https://bit.ly/2XhnF25
#[derive(Default, Debug)]
pub struct CachedSize {
    size: AtomicU32,
}

impl CachedSize {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get cached size
    #[inline]
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    /// Set cached size
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
