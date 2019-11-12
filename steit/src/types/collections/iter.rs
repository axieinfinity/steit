pub struct Iter<'a, T> {
    inner: Box<dyn Iterator<Item = &'a Option<T>> + 'a>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(inner: impl Iterator<Item = &'a Option<T>> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(Some(item)) => return Some(item),
                Some(None) => (),
                None => return None,
            }
        }
    }
}

pub struct IterMut<'a, T> {
    inner: Box<dyn Iterator<Item = &'a mut Option<T>> + 'a>,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub fn new(inner: impl Iterator<Item = &'a mut Option<T>> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(Some(item)) => return Some(item),
                Some(None) => (),
                None => return None,
            }
        }
    }
}
