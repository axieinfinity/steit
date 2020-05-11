use std::marker::PhantomData;

use super::key_v2::MapKeyV2;

pub struct IterV2<'a, K: MapKeyV2, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u32, &'a V)> + 'a>,
    _marker: PhantomData<*const K>,
}

impl<'a, K: MapKeyV2, V> IterV2<'a, K, V> {
    #[inline]
    pub(super) fn new(inner: impl Iterator<Item = (&'a u32, &'a V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: MapKeyV2, V> Iterator for IterV2<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&field_number, value)| {
            let key = K::try_from_field_number(field_number).unwrap();
            (key, value)
        })
    }
}

pub struct IterMutV2<'a, K: MapKeyV2, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u32, &'a mut V)> + 'a>,
    _marker: PhantomData<*const K>,
}

impl<'a, K: MapKeyV2, V> IterMutV2<'a, K, V> {
    #[inline]
    pub(super) fn new(inner: impl Iterator<Item = (&'a u32, &'a mut V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: MapKeyV2, V> Iterator for IterMutV2<'a, K, V> {
    type Item = (K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&field_number, value)| {
            let key = K::try_from_field_number(field_number).unwrap();
            (key, value)
        })
    }
}
