use std::{hash::Hash, io, iter::FromIterator, marker::PhantomData, ops};

use indexmap::map::IndexMap;
use serde::Serialize as JsonSerialize;

use crate::{
    de::{Deserialize, Reader},
    log::LogEntryKind,
    meta::{FieldTypeMeta, HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::{Runtime, SizeCache},
    ser::Serialize,
    state::State,
    wire_fmt::{self, HasWireType, WireType},
};

use super::{
    iter::{MapIter, MapIterMut},
    key::MapKey,
};

#[derive(Clone, Debug, JsonSerialize)]
pub struct Map<K: MapKey, V: State> {
    #[serde(skip_serializing)]
    entries: IndexMap<u32, V>,
    size_cache: SizeCache,
    runtime: Runtime,
    _marker: PhantomData<*const K>,
}

impl<K: MapKey, V: State> Map<K, V> {
    pub fn new(runtime: Runtime) -> Self {
        Self {
            entries: IndexMap::new(),
            size_cache: SizeCache::new(),
            runtime,
            _marker: PhantomData,
        }
    }

    pub fn from_iter(runtime: Runtime, iter: impl IntoIterator<Item = (K, V)>) -> Self {
        let mut map: Self = FromIterator::from_iter(iter);
        map.set_runtime(runtime);
        map
    }

    pub fn contains(&self, key: &K) -> bool
    where
        K: Eq + Hash,
    {
        self.entries.contains_key(&key.as_field_number())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(&key.as_field_number())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries.get_mut(&key.as_field_number())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let field_number = key.as_field_number();
        self.runtime.log_update_child(field_number, &value).unwrap();
        self.entries.insert(field_number, value)
    }

    pub fn insert_with(&mut self, key: K, get_value: impl FnOnce(Runtime) -> V) -> Option<V> {
        let tag = key.as_field_number();
        self.runtime.pause_logger();
        let value = get_value(self.runtime.nested(tag));
        self.runtime.unpause_logger();
        self.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let field_number = key.as_field_number();
        self.runtime.log_map_remove(field_number).unwrap();
        self.entries.remove(&field_number)
    }

    pub fn iter(&self) -> MapIter<K, V> {
        MapIter::new(self.entries.iter())
    }

    pub fn iter_mut(&mut self) -> MapIterMut<K, V> {
        MapIterMut::new(self.entries.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K: MapKey, V: State> ops::Index<&K> for Map<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).expect("no entry found for key")
    }
}

impl<K: MapKey, V: State> ops::IndexMut<&K> for Map<K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).expect("no entry found for key")
    }
}

impl<K: MapKey + Eq + Hash, V: State + PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl<K: MapKey + Eq + Hash, V: State + Eq> Eq for Map<K, V> {}

impl<K: MapKey, V: State> Default for Map<K, V> {
    fn default() -> Self {
        Self::new(Runtime::default())
    }
}

impl<K: MapKey, V: State> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self {
            entries: iter
                .into_iter()
                .map(|(key, value)| {
                    let key = key.as_field_number();
                    (key, value)
                })
                .collect(),
            ..Default::default()
        }
    }
}

impl<'a, K: MapKey, V: State> IntoIterator for &'a Map<K, V> {
    type Item = (K, &'a V);
    type IntoIter = MapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: MapKey, V: State> IntoIterator for &'a mut Map<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = MapIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: MapKey, V: State> HasWireType for Map<K, V> {
    const WIRE_TYPE: WireType = WireType::Sized;
}

impl<K: MapKey, V: State> Serialize for Map<K, V> {
    fn compute_size(&self) -> u32 {
        let mut size = 0;

        for (&field_number, value) in &self.entries {
            size += value.compute_size_nested(field_number, false).unwrap();
        }

        size
    }

    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for (&field_number, value) in &self.entries {
            value.serialize_nested(field_number, false, writer)?;
        }

        Ok(())
    }

    fn size_cache(&self) -> Option<&SizeCache> {
        Some(&self.size_cache)
    }
}

impl<K: MapKey, V: State> Deserialize for Map<K, V> {
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let field_number = u32::deserialize(reader)?;
            wire_fmt::validate_field_number(field_number)?;
            K::try_from_field_number(field_number)?;

            if let Some(value) = self.entries.get_mut(&field_number) {
                value.merge_nested(V::WIRE_TYPE, reader)?;
            } else {
                let mut value = V::with_runtime(self.runtime.nested(field_number));
                value.merge_nested(V::WIRE_TYPE, reader)?;
                self.entries.insert(field_number, value);
            }
        }

        Ok(())
    }
}

impl<K: MapKey, V: State> State for Map<K, V> {
    fn with_runtime(runtime: Runtime) -> Self {
        Self::new(runtime)
    }

    fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    fn set_runtime(&mut self, runtime: Runtime) {
        for (&field_number, value) in self.entries.iter_mut() {
            value.set_runtime(runtime.nested(field_number));
        }

        self.runtime = runtime;
    }

    fn handle(
        &mut self,
        mut path: impl Iterator<Item = u32>,
        kind: LogEntryKind,
        key: Option<u32>,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()> {
        if let Some(field_number) = path.next() {
            if let Some(value) = self.entries.get_mut(&field_number) {
                value.handle(path, kind, key, reader)
            } else if kind == LogEntryKind::Update && path.next().is_none() {
                let mut value = V::with_runtime(self.runtime.nested(field_number));
                value.merge_nested(V::WIRE_TYPE, reader)?;
                self.entries.insert(field_number, value);
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("key {} not found", field_number),
                ))
            }
        } else {
            match kind {
                LogEntryKind::Update => self.handle_update(reader),

                LogEntryKind::MapRemove => {
                    let key = key.ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "missing key for `LogEntryKind::MapInsert`",
                        )
                    })?;

                    if self.entries.remove(&key).is_some() {
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("key {} not found", key),
                        ))
                    }
                }

                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{:?} is not supported on `Map` (maybe on its items?)", kind),
                )),
            }
        }
    }
}

impl<K: MapKey, V: State + HasMeta> HasMeta for Map<K, V> {
    const NAME: &'static NameMeta = &NameMeta {
        rust: "Map",
        csharp: Some("StateMap"),
    };

    const TYPE: &'static TypeMeta = &TypeMeta::Ref(Self::NAME, &[FieldTypeMeta::Type(V::TYPE)]);

    const LINK: &'static MetaLink = &MetaLink {
        r#type: Self::TYPE,
        msg: None,
        links: || &[V::LINK],
    };
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{
        log::loggers::BufferLogger,
        rt::{LoggerHandle, Runtime},
        state::State,
        test_util::{assert_serialize, merge, replay, Point},
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
        fn try_from_field_number<'a>(tag: u32) -> io::Result<Self> {
            match tag {
                1 => Ok(Key::One),
                2 => Ok(Key::Two),
                3 => Ok(Key::Three),
                4 => Ok(Key::Four),
                5 => Ok(Key::Five),
                _ => Err(io::Error::new(io::ErrorKind::InvalidData, "unknown key")),
            }
        }

        fn as_field_number(&self) -> u32 {
            match self {
                Key::One => 1,
                Key::Two => 2,
                Key::Three => 3,
                Key::Four => 4,
                Key::Five => 5,
            }
        }
    }

    fn map_with_logger<K: MapKey, V: State>() -> (Map<K, V>, LoggerHandle<BufferLogger>) {
        let (runtime, logger) = Runtime::with_logger_returned(BufferLogger::new());
        let map = Map::new(runtime);
        (map, logger)
    }

    fn map<K: MapKey, V: State>() -> Map<K, V> {
        map_with_logger().0
    }

    #[test]
    fn insert_and_get() {
        let mut map = map();
        map.insert(17u8, 23);
        assert_eq!(map.get(&17), Some(&23));
    }

    #[test]
    fn insert_and_check_log_01() {
        let (mut map, logger) = map_with_logger();
        map.insert(1u16, 1);
        map.insert(2, 2);

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[7, 0, 2, 1, 1, 10, 1, 2, /**/ 7, 0, 2, 1, 2, 10, 1, 4]
        );
    }

    #[test]
    fn insert_and_check_log_02() {
        let (mut map, logger) = map_with_logger();
        map.insert_with(3u32, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(7, |runtime| Point::new(runtime, 2, 2, 2));

        assert_eq!(
            logger.lock().unwrap().bytes(),
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
        map.insert(1u32, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        logger.lock().unwrap().clear();
        map.remove(&2);
        assert_eq!(logger.lock().unwrap().bytes(), &[3, 12, 8, 2]);
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
        map.insert_with(Key::Two, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(Key::Four, |runtime| Point::new(runtime, 2, 2, 2));
        map.insert_with(Key::Five, |runtime| Point::new(runtime, 3, 3, 3));
        logger.lock().unwrap().clear();

        for (_, value) in map.iter_mut() {
            value.set_x(value.x + 1);
        }

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[
                8, 0, 2, 2, 2, 0, 10, 1, 0, /**/ 8, 0, 2, 2, 4, 0, 10, 1, 6, /**/ 8, 0,
                2, 2, 5, 0, 10, 1, 8
            ]
        );
    }

    #[test]
    fn serialize_01() {
        let mut map = map();
        map.insert(2u8, 20);
        map.insert(3, 30);
        map.remove(&2);
        map.insert(6, 60);
        map.insert(7, 70);
        assert_serialize(map, &[24, 60, 48, 120, 56, 140, 1]);
    }

    #[test]
    fn serialize_02() {
        let mut map = map();
        map.insert_with(Key::Two, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(Key::Three, |runtime| Point::new(runtime, 2, 2, 2));
        map.insert_with(Key::One, |runtime| Point::new(runtime, 3, 3, 3));
        map.remove(&Key::Three);

        assert_serialize(
            map,
            &[18, 6, 0, 1, 8, 1, 16, 1, /**/ 10, 6, 0, 6, 8, 6, 16, 6],
        );
    }

    #[test]
    fn merge_no_log() {
        let (mut map, logger) = map_with_logger();
        map.insert(1u16, 10);
        map.insert(2, 20);
        logger.lock().unwrap().clear();
        merge(&mut map, &[3, 60]);
        assert_eq!(map.get(&3), Some(&30));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn merge_update_nested() {
        let mut map = map();
        map.insert_with(1u8, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(10, |runtime| Point::new(runtime, 2, 2, 2));
        merge(&mut map, &[10, 2, 8, 5]);
        assert_eq!(map.get(&10), Some(&Point::new(Runtime::new(), 2, -3, 2)));
    }

    #[test]
    fn merge_push_new() {
        let mut map = map();
        merge(&mut map, &[2, 2, 16, 7]);
        assert_eq!(map.get(&2u8), Some(&Point::new(Runtime::new(), 0, 0, -4)));
    }

    #[test]
    fn replay_insert_no_log() {
        let (mut map, logger) = map_with_logger();
        replay(&mut map, &[7, 0, 2, 1, 7, 10, 1, 1]);
        assert_eq!(map.get(&7u8), Some(&-1));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn replay_update() {
        let mut map = map();
        map.insert(1u16, 111);
        replay(&mut map, &[7, 0, 2, 1, 1, 10, 1, 1]);
        assert_eq!(map.get(&1), Some(&-1));
    }

    #[test]
    fn replay_update_nested() {
        let mut map = map();
        map.insert_with(3u32, |runtime| Point::new(runtime, -1, -1, -1));
        replay(&mut map, &[8, 0, 2, 2, 3, 2, 10, 1, 100]);
        assert_eq!(map.get(&3), Some(&Point::new(Runtime::new(), -1, -1, 50)));
    }

    #[test]
    fn replay_remove() {
        let mut map = map();
        map.insert_with(10u32, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(15, |runtime| Point::new(runtime, 2, 2, 2));
        replay(&mut map, &[3, 12, 8, 10]);
        assert_eq!(map.get(&10), None);
        assert_eq!(map.get(&15), Some(&Point::new(Runtime::new(), 2, 2, 2)));
    }

    #[test]
    #[should_panic(expected = "key 1 not found")]
    fn replay_remove_key_not_found() {
        replay(&mut map::<u16, i32>(), &[4, 12, 2, 1, 1]);
    }
}
