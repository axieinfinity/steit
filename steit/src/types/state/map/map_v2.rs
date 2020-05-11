use std::{io, marker::PhantomData};

use indexmap::map::IndexMap;

use crate::{
    de_v2::{DeserializeV2, Reader},
    meta::{FieldTypeMeta, HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::{RuntimeV2, SizeCache},
    ser_v2::SerializeV2,
    state_v2::StateV2,
    wire_fmt::{self, HasWireType, WireTypeV2},
};

use super::{
    iter_v2::{IterMutV2, IterV2},
    key_v2::MapKeyV2,
};

#[derive(Debug)]
pub struct MapV2<K: MapKeyV2, V: StateV2> {
    entries: IndexMap<u32, V>,
    size_cache: SizeCache,
    runtime: RuntimeV2,
    _marker: PhantomData<*const K>,
}

impl<K: MapKeyV2, V: StateV2> MapV2<K, V> {
    #[inline]
    pub fn new(runtime: RuntimeV2) -> Self {
        Self {
            entries: IndexMap::new(),
            size_cache: SizeCache::new(),
            runtime,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(&key.as_field_number())
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries.get_mut(&key.as_field_number())
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let field_number = key.as_field_number();
        self.runtime.log_update_child(field_number, &value).unwrap();
        self.entries.insert(field_number, value)
    }

    #[inline]
    pub fn insert_with(&mut self, key: K, get_value: impl FnOnce(RuntimeV2) -> V) -> Option<V> {
        let tag = key.as_field_number();
        self.runtime.pause_logger();
        let value = get_value(self.runtime.nested(tag));
        self.runtime.unpause_logger();
        self.insert(key, value)
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let field_number = key.as_field_number();
        self.runtime.log_map_remove(field_number).unwrap();
        self.entries.remove(&field_number)
    }

    #[inline]
    pub fn iter(&self) -> IterV2<K, V> {
        IterV2::new(self.entries.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMutV2<K, V> {
        IterMutV2::new(self.entries.iter_mut())
    }
}

impl<K: MapKeyV2, V: StateV2> Default for MapV2<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new(RuntimeV2::default())
    }
}

impl<'a, K: MapKeyV2, V: StateV2> IntoIterator for &'a MapV2<K, V> {
    type Item = (K, &'a V);
    type IntoIter = IterV2<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: MapKeyV2, V: StateV2> IntoIterator for &'a mut MapV2<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMutV2<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: MapKeyV2, V: StateV2> HasWireType for MapV2<K, V> {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl<K: MapKeyV2, V: StateV2> SerializeV2 for MapV2<K, V> {
    #[inline]
    fn compute_size_v2(&self) -> u32 {
        let mut size = 0;

        for (&field_number, value) in &self.entries {
            size += value.compute_size_nested_v2(field_number, false).unwrap();
        }

        size
    }

    #[inline]
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        for (&field_number, value) in &self.entries {
            value.serialize_nested(field_number, false, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        Some(&self.size_cache)
    }
}

impl<K: MapKeyV2, V: StateV2> DeserializeV2 for MapV2<K, V> {
    #[inline]
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let field_number = u32::deserialize_v2(reader)?;
            wire_fmt::validate_field_number(field_number)?;
            K::try_from_field_number(field_number)?;

            if let Some(value) = self.entries.get_mut(&field_number) {
                value.merge_nested_v2(V::WIRE_TYPE, reader)?;
            } else {
                let mut value = V::with_runtime_v2(self.runtime.nested(field_number));
                value.merge_nested_v2(V::WIRE_TYPE, reader)?;
                self.entries.insert(field_number, value);
            }
        }

        Ok(())
    }
}

impl<K: MapKeyV2, V: StateV2> StateV2 for MapV2<K, V> {
    #[inline]
    fn with_runtime_v2(runtime: RuntimeV2) -> Self {
        Self::new(runtime)
    }

    #[inline]
    fn runtime_v2(&self) -> &RuntimeV2 {
        &self.runtime
    }

    #[inline]
    fn set_runtime_v2(&mut self, runtime: RuntimeV2) {
        for (&field_number, value) in self.entries.iter_mut() {
            value.set_runtime_v2(runtime.nested(field_number));
        }

        self.runtime = runtime;
    }
}

impl<K: MapKeyV2, V: StateV2 + HasMeta> HasMeta for MapV2<K, V> {
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
        log::loggers::BufferLoggerV2,
        rt::{LoggerHandleV2, RuntimeV2},
        state_v2::StateV2,
        test_util_v2::{assert_serialize, merge, Point},
    };

    use super::{MapKeyV2, MapV2};

    #[derive(PartialEq, Debug)]
    enum Key {
        One,
        Two,
        Three,
        Four,
        Five,
    }

    impl MapKeyV2 for Key {
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

    fn map_with_logger<K: MapKeyV2, V: StateV2>() -> (MapV2<K, V>, LoggerHandleV2<BufferLoggerV2>) {
        let (runtime, logger) = RuntimeV2::with_logger_returned(BufferLoggerV2::new());
        let map = MapV2::new(runtime);
        (map, logger)
    }

    fn map<K: MapKeyV2, V: StateV2>() -> MapV2<K, V> {
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

        assert_eq!(logger.lock().unwrap().bytes(), &[4, 12, 2, 1, 2]);
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

        for (_key, value) in map.iter_mut() {
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

        let map = merge(map, &[3, 60]);

        assert_eq!(map.get(&3), Some(&30));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn merge_update_nested() {
        let mut map = map();

        map.insert_with(1u8, |runtime| Point::new(runtime, -1, -1, -1));
        map.insert_with(10, |runtime| Point::new(runtime, 2, 2, 2));

        let map = merge(map, &[10, 2, 8, 5]);

        assert_eq!(map.get(&10), Some(&Point::new(RuntimeV2::new(), 2, -3, 2)));
    }

    #[test]
    fn merge_push_new() {
        let map = merge(map(), &[2, 2, 16, 7]);
        assert_eq!(map.get(&2u8), Some(&Point::new(RuntimeV2::new(), 0, 0, -4)));
    }

    // #[test]
    // fn replay_insert_no_log() {
    //     let (map, logger) = map_with_logger();
    //     let map = replay(map, &[7, 0, 2, 1, 7, 10, 1, 1]);
    //     assert_eq!(map.get(&7), Some(&-1));
    //     assert_eq!(logger.lock().unwrap().bytes(), &[]);
    // }
    //
    // #[test]
    // fn replay_update() {
    //     let mut map = map();
    //
    //     map.insert(1, 111);
    //
    //     let map = replay(map, &[7, 0, 2, 1, 1, 10, 1, 1]);
    //
    //     assert_eq!(map.get(&1), Some(&-1));
    // }
    //
    // #[test]
    // fn replay_update_nested() {
    //     let mut map = map();
    //
    //     map.insert_with(3, |runtime| Point::new(runtime, -1, -1, -1));
    //
    //     let map = replay(map, &[8, 0, 2, 2, 3, 2, 10, 1, 100]);
    //
    //     assert_eq!(map.get(&3), Some(&Point::new(RuntimeV2::new(), -1, -1, 50)));
    // }
    //
    // #[test]
    // fn replay_remove() {
    //     let mut map = map();
    //
    //     map.insert_with(10, |runtime| Point::new(runtime, -1, -1, -1));
    //     map.insert_with(15, |runtime| Point::new(runtime, 2, 2, 2));
    //
    //     let map = replay(map, &[4, 2, 2, 1, 10]);
    //
    //     assert_eq!(map.get(&10), None);
    //     assert_eq!(map.get(&15), Some(&Point::new(RuntimeV2::new(), 2, 2, 2)));
    // }
    //
    // #[test]
    // #[should_panic(expected = "nonexistent value with tag 1")]
    // fn replay_remove_nonexistent() {
    //     replay(map::<u16, i32>(), &[4, 2, 2, 1, 1]);
    // }
}
