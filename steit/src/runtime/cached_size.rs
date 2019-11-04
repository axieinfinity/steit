use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct CachedSize {
    size: AtomicU32,
}

impl CachedSize {
    const UNSET: u32 = -1i8 as u32;

    #[inline]
    pub fn new() -> Self {
        Self {
            size: AtomicU32::new(Self::UNSET),
        }
    }

    /// Get cached size
    #[inline]
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed)
    }

    /// Set cached size
    #[inline]
    pub fn set(&self, size: u32) {
        self.size.store(size, Ordering::Relaxed)
    }

    #[inline]
    pub fn get_or_set_from(&self, f: impl FnOnce() -> u32) -> u32 {
        match self.size.load(Ordering::Relaxed) {
            Self::UNSET => {
                let size = f();
                self.set(size);
                size
            }
            size => size,
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.set(Self::UNSET)
    }
}

impl Default for CachedSize {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CachedSize {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            size: AtomicU32::new(self.get()),
        }
    }
}

impl PartialEq for CachedSize {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CachedSize {}
