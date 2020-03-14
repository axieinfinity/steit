use std::io;

use crate::{
    gen::{FieldType, IsFieldType},
    wire_type::{self, WireType, WIRE_TYPE_SIZED},
    CachedSize, Deserialize, Eof, Merge, ReplayKind, Runtime, Serialize, State,
};

use super::{
    enumerate::{Enumerate, EnumerateMut},
    iter::{Iter, IterMut},
};

#[derive(Default, Debug)]
pub struct List<T: State> {
    items: Vec<Option<T>>,
    cached_size: CachedSize,
    runtime: Runtime,
}

impl<T: State> List<T> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            items: Vec::new(),
            cached_size: CachedSize::new(),
            runtime,
        }
    }

    #[inline]
    pub fn get(&self, tag: u16) -> Option<&T> {
        match self.items.get(tag as usize) {
            Some(item) => item.as_ref(),
            None => None,
        }
    }

    #[inline]
    pub fn get_mut(&mut self, tag: u16) -> Option<&mut T> {
        match self.items.get_mut(tag as usize) {
            Some(item) => item.as_mut(),
            None => None,
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.runtime.log_add(&item).unwrap();
        self.items.push(Some(item));
    }

    #[inline]
    pub fn push_with(&mut self, f: impl FnOnce(Runtime) -> T) {
        let tag = self.items.len();

        assert!(
            tag <= u16::max_value() as usize,
            "`List` indices must be within `u16`"
        );

        self.runtime.pause_logger();
        let item = f(self.runtime.nested(tag as u16));
        self.runtime.unpause_logger();
        self.push(item);
    }

    #[inline]
    pub fn remove(&mut self, tag: u16) -> Option<T> {
        self.runtime.log_remove(tag).unwrap();
        self.items.remove(tag as usize)
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.items.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self.items.iter_mut())
    }

    #[inline]
    pub fn enumerate(&self) -> Enumerate<T> {
        Enumerate::new(self.items.iter())
    }

    #[inline]
    pub fn enumerate_mut(&mut self) -> EnumerateMut<T> {
        EnumerateMut::new(self.items.iter_mut())
    }
}

impl<'a, T: State> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: State> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: State> WireType for List<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: State> Serialize for List<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        assert!(
            self.items.len() <= u16::max_value() as usize + 1,
            "`List` indices must be within `u16`"
        );

        let mut size = 0;

        for (tag, item) in self.items.iter().enumerate() {
            if let Some(item) = item {
                size += item.compute_size_nested_omittable(tag as u16, false);
            }
        }

        self.cached_size.set(size);
        size
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        assert!(
            self.items.len() <= u16::max_value() as usize + 1,
            "`List` indices must be within `u16`"
        );

        for (tag, item) in self.items.iter().enumerate() {
            if let Some(item) = item {
                item.serialize_nested_omittable_with_cached_size(tag as u16, false, writer)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.cached_size.get()
    }
}

impl<T: State> Merge for List<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let key = u32::deserialize(reader)?;
            let (tag, wire_type) = wire_type::split_key(key);

            if wire_type == T::WIRE_TYPE {
                if tag as usize >= self.items.len() {
                    self.items.resize_with(tag as usize + 1, || None);
                }

                let item = self.items.get_mut(tag as usize).unwrap();

                if item.is_none() {
                    *item = Some(T::with_runtime(self.runtime.nested(tag)));
                }

                if let Some(item) = item.as_mut() {
                    item.merge_nested(reader)?;
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

impl<T: State> State for List<T> {
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
            if (tag as usize) < self.items.len() {
                if let Some(item) = &mut self.items[tag as usize] {
                    let path = &mut path.peekable();

                    if kind == &ReplayKind::Remove && path.peek().is_none() {
                        self.items[tag as usize] = None;
                        Ok(())
                    } else {
                        item.handle(path, kind, reader)
                    }
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("nonexistent item with tag {}", tag),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("out-of-bounds tag {}", tag),
                ))
            }
        } else {
            match kind {
                ReplayKind::Update => self.handle_update(reader),

                ReplayKind::Add => {
                    let item = T::deserialize(reader)?;
                    self.items.push(Some(item));
                    Ok(())
                }

                ReplayKind::Remove => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`remove` are not supported on the current `List` but its items",
                )),
            }
        }
    }
}

impl<T: State + IsFieldType> IsFieldType for List<T> {
    const FIELD_TYPE: &'static FieldType = &FieldType::List(T::FIELD_TYPE);
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{
        log::loggers::BufferLogger,
        test_utils::{assert_serialize, merge, replay, Point},
        Runtime, State,
    };

    use super::List;

    fn list_with_logger<T: State>() -> (List<T>, Arc<Mutex<BufferLogger>>) {
        let logger = Arc::new(Mutex::new(BufferLogger::new()));
        let list = List::new(Runtime::with_logger(Box::new(logger.clone())));
        (list, logger)
    }

    fn list<T: State>() -> List<T> {
        list_with_logger().0
    }

    #[test]
    fn push_and_get() {
        let mut list = list();
        list.push(1);
        assert_eq!(list.get(0), Some(&1));
    }

    #[test]
    fn push_and_check_log_01() {
        let (mut list, logger) = list_with_logger();

        list.push(1);
        list.push(2);

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[4, 1, 10, 1, 2, /**/ 4, 1, 10, 1, 4]
        );
    }

    #[test]
    fn push_and_check_log_02() {
        let (mut list, logger) = list_with_logger();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::with(runtime, 2, 2, 2));

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[9, 1, 10, 6, 0, 1, 8, 1, 16, 1, /**/ 9, 1, 10, 6, 0, 4, 8, 4, 16, 4]
        );
    }

    #[test]
    fn remove_and_get() {
        let mut list = list();

        list.push(0);
        list.push(1);

        list.remove(0);

        assert_eq!(list.get(0), None);
    }

    #[test]
    fn remove_and_check_log() {
        let (mut list, logger) = list_with_logger();

        list.push(1);
        list.push(2);
        list.push(3);

        logger.lock().unwrap().clear();

        list.remove(2);

        assert_eq!(logger.lock().unwrap().bytes(), &[4, 2, 2, 1, 2]);
    }

    #[test]
    fn iter() {
        let mut list = list();

        list.push(1);
        list.push(2);
        list.push(3);

        list.remove(1);

        assert_eq!(&list.iter().collect::<Vec<_>>(), &[&1, &3]);
    }

    #[test]
    fn iter_mut_update_and_check_log() {
        let (mut list, logger) = list_with_logger();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::with(runtime, 2, 2, 2));
        list.push_with(|runtime| Point::with(runtime, 3, 3, 3));

        logger.lock().unwrap().clear();

        for item in list.iter_mut() {
            item.set_x(item.x() + 1);
        }

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[
                8, 0, 2, 2, 0, 0, 10, 1, 0, /**/ 8, 0, 2, 2, 1, 0, 10, 1, 6, /**/ 8, 0,
                2, 2, 2, 0, 10, 1, 8
            ]
        );
    }

    #[test]
    fn serialize_01() {
        let mut list = list();

        list.push(1);
        list.push(0);

        list.remove(0);

        list.push(2);
        list.push(3);

        assert_serialize(list, &[8, 0, 16, 4, 24, 6]);
    }

    #[test]
    fn serialize_02() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::with(runtime, 2, 2, 2));
        list.push_with(|runtime| Point::with(runtime, 3, 3, 3));

        list.remove(1);

        assert_serialize(
            list,
            &[2, 6, 0, 1, 8, 1, 16, 1, /**/ 18, 6, 0, 6, 8, 6, 16, 6],
        );
    }

    #[test]
    fn merge_no_log() {
        let (mut list, logger) = list_with_logger();

        list.push(10);
        list.push(20);

        logger.lock().unwrap().clear();

        let list = merge(list, &[40, 60]);

        assert_eq!(list.get(5), Some(&30));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn merge_update_nested() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::with(runtime, 2, 2, 2));

        let list = merge(list, &[2, 2, 8, 5]);

        assert_eq!(list.get(0), Some(&Point::with(Runtime::new(), -1, -3, -1)));
    }

    #[test]
    fn merge_push_new() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));

        let list = merge(list, &[18, 2, 16, 7]);

        assert_eq!(list.get(1), None);
        assert_eq!(list.get(2), Some(&Point::with(Runtime::new(), 0, 0, -4)));
    }

    #[test]
    fn replay_push_no_log() {
        let (list, logger) = list_with_logger();
        let list = replay(list, &[4, 1, 10, 1, 1]);
        assert_eq!(list.get(0), Some(&-1));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn replay_push_next_index() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));

        let list = merge(list, &[18, 2, 16, 7]);
        let list = replay(list, &[9, 1, 10, 6, 0, 3, 8, 2, 16, 10]);

        assert_eq!(list.get(3), Some(&Point::with(Runtime::new(), -2, 1, 5)));
    }

    #[test]
    #[should_panic(expected = "out-of-bounds tag 0")]
    fn replay_update_out_of_bounds() {
        replay(list::<i32>(), &[7, 0, 2, 1, 0, 10, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "nonexistent item with tag 1")]
    fn replay_update_nonexistent() {
        let mut list = list();

        list.push(1);
        list.push(2);
        list.remove(1);

        replay(list, &[7, 0, 2, 1, 1, 10, 1, 1]);
    }

    #[test]
    fn replay_update() {
        let mut list = list();

        list.push(0);

        let list = replay(list, &[7, 0, 2, 1, 0, 10, 1, 1]);

        assert_eq!(list.get(0), Some(&-1));
    }

    #[test]
    fn replay_update_nested() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));

        let list = replay(list, &[8, 0, 2, 2, 0, 2, 10, 1, 100]);

        assert_eq!(list.get(0), Some(&Point::with(Runtime::new(), -1, -1, 50)));
    }

    #[test]
    fn replay_remove() {
        let mut list = list();

        list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::with(runtime, 2, 2, 2));

        let list = replay(list, &[4, 2, 2, 1, 0]);

        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), Some(&Point::with(Runtime::new(), 2, 2, 2)));
    }
}
