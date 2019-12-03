use std::{io, marker::PhantomData};

use indexmap::map::IndexMap;

use crate::{
    wire_type::{self, WireType, WIRE_TYPE_SIZED},
    CachedSize, Deserialize, Eof, Merge, ReplayKind, Runtime, Serialize, State,
};

use super::{
    iter::{Iter, IterMut},
    key::MapKey,
};

#[derive(Debug)]
pub struct Map<K: MapKey, V: State> {
    entries: IndexMap<u16, V>,
    phantom: PhantomData<*const K>,
    cached_size: CachedSize,
    runtime: Runtime,
}

impl<K: MapKey, V: State> Map<K, V> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            entries: IndexMap::new(),
            phantom: PhantomData,
            cached_size: CachedSize::new(),
            runtime,
        }
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(&key.as_tag())
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries.get_mut(&key.as_tag())
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        let tag = key.as_tag();
        self.runtime.log_update(tag, &value).unwrap();
        self.entries.insert(tag, value);
    }

    #[inline]
    pub fn insert_with(&mut self, key: K, f: impl FnOnce(Runtime) -> V) {
        let tag = key.as_tag();
        let value = f(self.runtime.nested(tag));
        self.insert(key, value);
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let tag = key.as_tag();
        self.runtime.log_remove(tag).unwrap();
        self.entries.remove(&tag)
    }

    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(self.entries.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut::new(self.entries.iter_mut())
    }
}

impl<K: MapKey, V: State> Default for Map<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new(Runtime::default())
    }
}

impl<'a, K: MapKey, V: State> IntoIterator for &'a Map<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: MapKey, V: State> IntoIterator for &'a mut Map<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: MapKey, V: State> WireType for Map<K, V> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<K: MapKey, V: State> Serialize for Map<K, V> {
    #[inline]
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for (&tag, value) in &self.entries {
            size += value.compute_size_nested_omittable(tag, false);
        }

        self.cached_size.set(size);
        size
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for (&tag, value) in &self.entries {
            value.serialize_nested_with_cached_size_omittable(tag, false, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.cached_size.get()
    }
}

impl<K: MapKey, V: State> Merge for Map<K, V> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let key = u32::deserialize(reader)?;
            let (tag, wire_type) = wire_type::split_key(key);

            if wire_type == V::WIRE_TYPE {
                let value = self.entries.get_mut(&tag);

                if value.is_none() {
                    self.entries
                        .insert(tag, V::with_runtime(self.runtime.nested(tag)));
                }

                if let Some(value) = self.entries.get_mut(&tag) {
                    value.merge_nested(reader)?;
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unexpected wire type {}", wire_type),
                ));
            }
        }

        Ok(())
    }
}

impl<K: MapKey, V: State> State for Map<K, V> {
    #[inline]
    fn with_runtime(runtime: Runtime) -> Self {
        Self::new(runtime)
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    #[inline]
    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()> {
        if let Some(&tag) = path.next() {
            let path = &mut path.peekable();

            if kind == &ReplayKind::Update && path.peek().is_none() {
                let mut value = V::with_runtime(self.runtime().nested(tag));
                value.merge(reader)?;
                self.entries.insert(tag, value);
                Ok(())
            } else if let Some(value) = self.entries.get_mut(&tag) {
                if kind == &ReplayKind::Remove && path.peek().is_none() {
                    self.entries.remove(&tag);
                    Ok(())
                } else {
                    value.handle(path, kind, reader)
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("nonexistent value with tag {}", tag),
                ))
            }
        } else {
            match kind {
                ReplayKind::Update => self.handle_update(reader),

                ReplayKind::Add | ReplayKind::Remove => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`add` and `remove` are not supported on the current `Map` but its items",
                )),
            }
        }
    }
}
