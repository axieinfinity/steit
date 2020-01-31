use std::{io, marker::PhantomData};

use indexmap::map::IndexMap;

use crate::{
    gen::{FieldType, IsFieldType},
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
        self.runtime.pause_logger();
        let value = f(self.runtime.nested(tag));
        self.runtime.unpause_logger();
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
            value.serialize_nested_omittable_with_cached_size(tag, false, writer)?;
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

impl<K: MapKey, T: State + IsFieldType> IsFieldType for Map<K, T> {
    const FIELD_TYPE: &'static FieldType = &FieldType::Map(T::FIELD_TYPE);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        log::loggers::BufferLogger,
        test_utils::{assert_serialize, merge, replay, Point},
        Runtime, State,
    };

    use super::{Map, MapKey};

    #[derive(PartialEq, Debug)]
    enum Key {
        One,
        Two,
        Three,
        Four,
        Five,
    }

    impl MapKey for Key {
        fn as_tag(&self) -> u16 {
            match self {
                Key::One => 1,
                Key::Two => 2,
                Key::Three => 3,
                Key::Four => 4,
                Key::Five => 5,
            }
        }

        fn try_from_tag<'a>(tag: u16) -> Result<Self, &'a str> {
            match tag {
                1 => Ok(Key::One),
                2 => Ok(Key::Two),
                3 => Ok(Key::Three),
                4 => Ok(Key::Four),
                5 => Ok(Key::Five),
                _ => Err("unknown key"),
            }
        }
    }

    fn map_with_logger<K: MapKey, V: State>() -> (Map<K, V>, Rc<RefCell<BufferLogger>>) {
        let logger = Rc::new(RefCell::new(BufferLogger::new()));
        let map = Map::new(Runtime::with_logger(Box::new(logger.clone())));
        (map, logger)
    }

    fn map<K: MapKey, V: State>() -> Map<K, V> {
        map_with_logger().0
    }

    #[test]
    fn insert_and_get() {
        let mut map = map();
        map.insert(17, 23);
        assert_eq!(map.get(&17), Some(&23));
    }

    #[test]
    fn insert_and_check_log_01() {
        let (mut map, logger) = map_with_logger();

        map.insert(1, 1);
        map.insert(2, 2);

        assert_eq!(
            logger.borrow().bytes(),
            &[7, 0, 2, 1, 1, 10, 1, 2, /**/ 7, 0, 2, 1, 2, 10, 1, 4]
        );
    }

    #[test]
    fn insert_and_check_log_02() {
        let (mut map, logger) = map_with_logger();

        map.insert_with(3, |runtime| Point::with(runtime, -1, -1, -1));
        map.insert_with(7, |runtime| Point::with(runtime, 2, 2, 2));

        assert_eq!(
            logger.borrow().bytes(),
            &[
                12, 0, 2, 1, 3, 10, 6, 0, 1, 8, 1, 16, 1, /**/ 12, 0, 2, 1, 7, 10, 6, 0, 4, 8,
                4, 16, 4
            ]
        );
    }

    #[test]
    fn remove_and_get() {
        let mut map = map();

        map.insert(Key::Two, 0);
        map.insert(Key::Three, 1);

        map.remove(&Key::Three);

        assert_eq!(map.get(&Key::Three), None);
    }

    #[test]
    fn remove_and_check_log() {
        let (mut map, logger) = map_with_logger();

        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        logger.borrow_mut().clear();

        map.remove(&2);

        assert_eq!(logger.borrow().bytes(), &[4, 2, 2, 1, 2]);
    }

    #[test]
    fn iter() {
        let mut map = map();

        map.insert(Key::One, 10);
        map.insert(Key::Two, 20);
        map.insert(Key::Three, 30);

        map.remove(&Key::One);

        assert_eq!(
            &map.iter().collect::<Vec<_>>(),
            &[(Key::Three, &30), (Key::Two, &20)]
        );
    }

    #[test]
    fn iter_mut_update_and_check_log() {
        let (mut map, logger) = map_with_logger();

        map.insert_with(Key::Two, |runtime| Point::with(runtime, -1, -1, -1));
        map.insert_with(Key::Four, |runtime| Point::with(runtime, 2, 2, 2));
        map.insert_with(Key::Five, |runtime| Point::with(runtime, 3, 3, 3));

        logger.borrow_mut().clear();

        for (_key, value) in map.iter_mut() {
            value.set_x(value.x() + 1);
        }

        assert_eq!(
            logger.borrow().bytes(),
            &[
                8, 0, 2, 2, 2, 0, 10, 1, 0, /**/ 8, 0, 2, 2, 4, 0, 10, 1, 6, /**/ 8, 0,
                2, 2, 5, 0, 10, 1, 8
            ]
        );
    }

    #[test]
    fn serialize_01() {
        let mut map = map();

        map.insert(2, 20);
        map.insert(3, 30);

        map.remove(&2);

        map.insert(6, 60);
        map.insert(7, 70);

        assert_serialize(map, &[24, 60, 48, 120, 56, 140, 1]);
    }

    #[test]
    fn serialize_02() {
        let mut map = map();

        map.insert_with(Key::Two, |runtime| Point::with(runtime, -1, -1, -1));
        map.insert_with(Key::Three, |runtime| Point::with(runtime, 2, 2, 2));
        map.insert_with(Key::One, |runtime| Point::with(runtime, 3, 3, 3));

        map.remove(&Key::Three);

        assert_serialize(
            map,
            &[18, 6, 0, 1, 8, 1, 16, 1, /**/ 10, 6, 0, 6, 8, 6, 16, 6],
        );
    }

    #[test]
    fn merge_no_log() {
        let (mut map, logger) = map_with_logger();

        map.insert(1, 10);
        map.insert(2, 20);

        logger.borrow_mut().clear();

        let map = merge(map, &[24, 60]);

        assert_eq!(map.get(&3), Some(&30));
        assert_eq!(logger.borrow().bytes(), &[]);
    }

    #[test]
    fn merge_update_nested() {
        let mut map = map();

        map.insert_with(1, |runtime| Point::with(runtime, -1, -1, -1));
        map.insert_with(10, |runtime| Point::with(runtime, 2, 2, 2));

        let map = merge(map, &[82, 2, 8, 5]);

        assert_eq!(map.get(&10), Some(&Point::with(Runtime::new(), 2, -3, 2)));
    }

    #[test]
    fn merge_push_new() {
        let map = merge(map(), &[18, 2, 16, 7]);
        assert_eq!(map.get(&2), Some(&Point::with(Runtime::new(), 0, 0, -4)));
    }

    #[test]
    fn replay_insert_no_log() {
        let (map, logger) = map_with_logger();
        let map = replay(map, &[7, 0, 2, 1, 7, 10, 1, 1]);
        assert_eq!(map.get(&7), Some(&-1));
        assert_eq!(logger.borrow().bytes(), &[]);
    }

    #[test]
    fn replay_update() {
        let mut map = map();

        map.insert(1, 111);

        let map = replay(map, &[7, 0, 2, 1, 1, 10, 1, 1]);

        assert_eq!(map.get(&1), Some(&-1));
    }

    #[test]
    fn replay_update_nested() {
        let mut map = map();

        map.insert_with(3, |runtime| Point::with(runtime, -1, -1, -1));

        let map = replay(map, &[8, 0, 2, 2, 3, 2, 10, 1, 100]);

        assert_eq!(map.get(&3), Some(&Point::with(Runtime::new(), -1, -1, 50)));
    }

    #[test]
    fn replay_remove() {
        let mut map = map();

        map.insert_with(10, |runtime| Point::with(runtime, -1, -1, -1));
        map.insert_with(15, |runtime| Point::with(runtime, 2, 2, 2));

        let map = replay(map, &[4, 2, 2, 1, 10]);

        assert_eq!(map.get(&10), None);
        assert_eq!(map.get(&15), Some(&Point::with(Runtime::new(), 2, 2, 2)));
    }

    #[test]
    #[should_panic(expected = "nonexistent value with tag 1")]
    fn replay_remove_nonexistent() {
        replay(map::<u16, i32>(), &[4, 2, 2, 1, 1]);
    }
}
