pub struct Enumerate<'a, T: 'a> {
    iter: Box<dyn Iterator<Item = &'a Option<T>> + 'a>,
    count: u16,
}

impl<'a, T> Enumerate<'a, T> {
    #[inline]
    pub(super) fn new(iter: impl Iterator<Item = &'a Option<T>> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
            count: 0,
        }
    }
}

impl<'a, T> Iterator for Enumerate<'a, T> {
    type Item = (u16, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.count;
            self.count += 1;

            match self.iter.next() {
                Some(Some(item)) => return Some((index, item)),
                Some(None) => (),
                None => return None,
            }
        }
    }
}

pub struct EnumerateMut<'a, T: 'a> {
    iter: Box<dyn Iterator<Item = &'a mut Option<T>> + 'a>,
    count: u16,
}

impl<'a, T> EnumerateMut<'a, T> {
    #[inline]
    pub(super) fn new(iter: impl Iterator<Item = &'a mut Option<T>> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
            count: 0,
        }
    }
}

impl<'a, T> Iterator for EnumerateMut<'a, T> {
    type Item = (u16, &'a mut T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.count;
            self.count += 1;

            match self.iter.next() {
                Some(Some(item)) => return Some((index, item)),
                Some(None) => (),
                None => return None,
            }
        }
    }
}
