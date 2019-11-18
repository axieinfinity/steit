use std::{collections::HashMap, convert::TryFrom, io, marker::PhantomData};

use crate::{
    wire_type::{self, WireType, WIRE_TYPE_SIZED},
    CachedSize, Deserialize, Eof, Merge, ReplayKind, Runtime, Serialize, State,
};

#[derive(Debug)]
pub struct Map<K: TryFrom<u16> + AsRef<u16>, V: State> {
    entries: HashMap<u16, V>,
    phantom: PhantomData<*const K>,
    cached_size: CachedSize,
    runtime: Runtime,
}

impl<K: TryFrom<u16> + AsRef<u16>, V: State> Map<K, V> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            entries: HashMap::new(),
            phantom: PhantomData,
            cached_size: CachedSize::new(),
            runtime,
        }
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        let tag = *key.as_ref();
        self.runtime.log_update(tag, &value).unwrap();
        self.entries.insert(tag, value);
    }

    #[inline]
    pub fn insert_with(&mut self, key: K, f: impl FnOnce(Runtime) -> V) {
        let tag = *key.as_ref();
        let value = f(self.runtime.nested(tag));
        self.insert(key, value);
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let tag = *key.as_ref();
        self.runtime.log_remove(tag).unwrap();
        self.entries.remove(&tag)
    }
}

impl<K: TryFrom<u16> + AsRef<u16>, V: State> Default for Map<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new(Runtime::default())
    }
}

impl<K: TryFrom<u16> + AsRef<u16>, V: State> WireType for Map<K, V> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<K: TryFrom<u16> + AsRef<u16>, V: State> Serialize for Map<K, V> {
    #[inline]
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for (&tag, value) in &self.entries {
            size += value.compute_size_nested(tag);
        }

        self.cached_size.set(size);
        size
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for (&tag, value) in &self.entries {
            value.serialize_nested_with_cached_size(tag, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.cached_size.get()
    }
}

impl<K: TryFrom<u16> + AsRef<u16>, V: State> Merge for Map<K, V> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let key = u32::deserialize(reader)?;
            let (tag, wire_type) = wire_type::split_key(key);

            if wire_type == V::WIRE_TYPE {
                let value = V::deserialize_nested(reader)?;
                self.entries.insert(tag, value);
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

impl<K: TryFrom<u16> + AsRef<u16>, V: State> State for Map<K, V> {
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
