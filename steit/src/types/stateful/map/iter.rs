use std::marker::PhantomData;

use super::key::MapKey;

pub struct Iter<'a, K: MapKey, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u16, &'a V)> + 'a>,
    phantom: PhantomData<*const K>,
}

impl<'a, K: MapKey, V> Iter<'a, K, V> {
    #[inline]
    pub(super) fn new(inner: impl Iterator<Item = (&'a u16, &'a V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            phantom: PhantomData,
        }
    }
}

impl<'a, K: MapKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&tag, value)| {
            let key = K::try_from_tag(tag).unwrap();
            (key, value)
        })
    }
}

pub struct IterMut<'a, K: MapKey, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u16, &'a mut V)> + 'a>,
    phantom: PhantomData<*const K>,
}

impl<'a, K: MapKey, V> IterMut<'a, K, V> {
    #[inline]
    pub(super) fn new(inner: impl Iterator<Item = (&'a u16, &'a mut V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            phantom: PhantomData,
        }
    }
}

impl<'a, K: MapKey, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&tag, value)| {
            let key = K::try_from_tag(tag).unwrap();
            (key, value)
        })
    }
}
