use std::cell::RefCell;

#[derive(Debug)]
pub struct CachedSize {
    size: RefCell<u32>,
}

impl CachedSize {
    const UNSET: u32 = -1i8 as u32;

    #[inline]
    pub fn unset() -> Self {
        Self {
            size: RefCell::new(Self::UNSET),
        }
    }

    /// Get cached size
    #[inline]
    pub fn get(&self) -> u32 {
        *self.size.borrow()
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        self.get() != Self::UNSET
    }

    /// Set cached size
    #[inline]
    pub fn set(&self, size: u32) {
        self.size.replace(size);
    }

    #[inline]
    pub fn get_or_set_from(&self, f: impl FnOnce() -> u32) -> u32 {
        let size = &mut *self.size.borrow_mut();

        match *size {
            Self::UNSET => {
                *size = f();
                *size
            }
            size => size,
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.set(Self::UNSET)
    }
}
