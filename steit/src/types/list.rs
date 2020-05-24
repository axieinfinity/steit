use std::{
    io,
    iter::FromIterator,
    ops::{self, Deref},
    slice,
};

use crate::{
    de::{Deserialize, Reader},
    log::LogEntryKind,
    meta::{FieldTypeMeta, HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::{Runtime, SizeCache},
    ser::Serialize,
    state::State,
    wire_fmt::{HasWireType, WireType},
};

#[derive(Clone, PartialEq, Eq, Default, Hash, Debug)]
pub struct List<T: State> {
    items: Vec<T>,
    size_cache: SizeCache,
    runtime: Runtime,
}

impl<T: State> Deref for List<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T: State> List<T> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            ..Default::default()
        }
    }

    pub fn from_iter(runtime: Runtime, iter: impl IntoIterator<Item = T>) -> Self {
        let mut list: Self = FromIterator::from_iter(iter);
        list.set_runtime(runtime);
        list
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    #[inline]
    pub fn push(&mut self, mut item: T) -> usize {
        self.push_with(|runtime| {
            item.set_runtime(runtime);
            item
        })
    }

    pub fn push_with(&mut self, get_item: impl FnOnce(Runtime) -> T) -> usize {
        let field_number = self.items.len() as u32;

        self.runtime.pause_logger();
        let item = get_item(self.runtime.nested(field_number));
        self.runtime.unpause_logger();

        self.runtime.log_list_push(&item).unwrap();
        self.items.push(item);

        field_number as usize
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            self.runtime.log_list_pop().unwrap();
            self.items.pop()
        } else {
            None
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.items.len() {
            return None;
        }

        let last_index = self.items.len() - 1;

        if index == last_index {
            return self.pop();
        }

        let runtime = &self.runtime;

        runtime
            .log_multi(vec![
                runtime.entry_list_pop(),
                runtime.entry_update_child(index as u32, &self.items[last_index]),
            ])
            .unwrap();

        Some(self.items.swap_remove(index))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.items.iter_mut()
    }
}

impl<T: State> ops::Index<usize> for List<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: State> ops::IndexMut<usize> for List<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<T: State> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl<'a, T: State> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T: State> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: State> HasWireType for List<T> {
    const WIRE_TYPE: WireType = WireType::Sized;
}

impl<T: State> Serialize for List<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.items.compute_size()
    }

    #[inline]
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.items.serialize_cached(writer)
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        Some(&self.size_cache)
    }
}

impl<T: State> Deserialize for List<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        let mut field_number = self.items.len() as u32;

        while !reader.eof()? {
            let mut item = T::with_runtime(self.runtime.nested(field_number));
            field_number += 1;
            item.merge_nested(T::WIRE_TYPE, reader)?;
            self.items.push(item);
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

    fn set_runtime(&mut self, runtime: Runtime) {
        for (field_number, item) in self.items.iter_mut().enumerate() {
            item.set_runtime(runtime.nested(field_number as u32))
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
            if let Some(item) = self.items.get_mut(field_number as usize) {
                item.handle(path, kind, key, reader)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("index {} out of bounds", field_number),
                ))
            }
        } else {
            match kind {
                LogEntryKind::Update => self.handle_update(reader),

                LogEntryKind::ListPush => {
                    let field_number = self.items.len() as u32;
                    let mut item = T::with_runtime(self.runtime.nested(field_number));
                    item.merge(reader)?;
                    self.items.push(item);
                    Ok(())
                }

                LogEntryKind::ListPop => {
                    if !self.items.is_empty() {
                        self.items.remove(self.items.len() - 1);
                        Ok(())
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "cannot pop from an empty `List`",
                        ))
                    }
                }

                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "{:?} is not supported on `List` (maybe on its items?)",
                        kind,
                    ),
                )),
            }
        }
    }
}

impl<T: State + HasMeta> HasMeta for List<T> {
    const NAME: &'static NameMeta = &NameMeta {
        rust: "List",
        csharp: Some("StateList"),
    };

    const TYPE: &'static TypeMeta = &TypeMeta::Ref(Self::NAME, &[FieldTypeMeta::Type(T::TYPE)]);

    const LINK: &'static MetaLink = &MetaLink {
        r#type: Self::TYPE,
        msg: None,
        links: || &[T::LINK],
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        log::loggers::BufferLogger,
        rt::{LoggerHandle, Runtime},
        state::State,
        test_util::{assert_serialize, merge, replay, Point},
    };

    use super::List;

    fn list_with_logger<T: State>() -> (List<T>, LoggerHandle<BufferLogger>) {
        let (runtime, logger) = Runtime::with_logger_returned(BufferLogger::new());
        let list = List::new(runtime);
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
            &[4, 8, 10, 1, 2, /**/ 4, 8, 10, 1, 4],
        );
    }

    #[test]
    fn push_and_check_log_02() {
        let (mut list, logger) = list_with_logger();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::new(runtime, 2, 2, 2));

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[9, 8, 10, 6, 0, 1, 8, 1, 16, 1, /**/ 9, 8, 10, 6, 0, 4, 8, 4, 16, 4],
        );
    }

    #[test]
    fn remove_and_get() {
        let mut list = list();
        list.push(0);
        list.push(1);
        list.swap_remove(0);
        assert_eq!(list.get(0), Some(&1));
    }

    #[test]
    fn remove_and_check_log() {
        let (mut list, logger) = list_with_logger();
        list.push(1);
        list.push(2);
        list.push(3);
        logger.lock().unwrap().clear();
        list.swap_remove(1);

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[1, 9, 7, 0, 2, 1, 1, 10, 1, 6],
        );
    }

    #[test]
    fn iter() {
        let mut list = list();
        list.push(1);
        list.push(2);
        list.push(3);
        list.swap_remove(0);
        assert_eq!(&list.iter().collect::<Vec<_>>(), &[&3, &2]);
    }

    #[test]
    fn iter_mut_update_and_check_log() {
        let (mut list, logger) = list_with_logger();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::new(runtime, 2, 2, 2));
        list.push_with(|runtime| Point::new(runtime, 3, 3, 3));
        logger.lock().unwrap().clear();

        for item in list.iter_mut() {
            item.set_x(item.x + 1);
        }

        assert_eq!(
            logger.lock().unwrap().bytes(),
            &[
                8, 0, 2, 2, 0, 0, 10, 1, 0, /**/ 8, 0, 2, 2, 1, 0, 10, 1, 6, /**/ 8, 0,
                2, 2, 2, 0, 10, 1, 8
            ],
        );
    }

    #[test]
    fn serialize_01() {
        let mut list = list();
        list.push(1);
        list.push(0);
        list.swap_remove(0);
        list.push(2);
        list.push(3);
        assert_serialize(list, &[0, 4, 6]);
    }

    #[test]
    fn serialize_02() {
        let mut list = list();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::new(runtime, 2, 2, 2));
        list.push_with(|runtime| Point::new(runtime, 3, 3, 3));
        list.swap_remove(1);
        assert_serialize(list, &[6, 0, 1, 8, 1, 16, 1, /**/ 6, 0, 6, 8, 6, 16, 6]);
    }

    #[test]
    fn merge_no_log() {
        let (mut list, logger) = list_with_logger();
        list.push(10);
        list.push(20);
        logger.lock().unwrap().clear();
        merge(&mut list, &[40, 60]);
        assert_eq!(list.get(3), Some(&30));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn merge_push_new() {
        let mut list = list();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        merge(&mut list, &[2, 16, 7]);
        assert_eq!(list.get(1), Some(&Point::new(Runtime::new(), 0, 0, -4)));
    }

    #[test]
    fn replay_push_no_log() {
        let (mut list, logger) = list_with_logger();
        replay(&mut list, &[4, 8, 10, 1, 1]);
        assert_eq!(list.get(0), Some(&-1));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn replay_push_next_index() {
        let mut list = list();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        replay(&mut list, &[9, 8, 10, 6, 0, 3, 8, 2, 16, 10]);
        assert_eq!(list.get(1), Some(&Point::new(Runtime::new(), -2, 1, 5)));
    }

    #[test]
    #[should_panic(expected = "index 0 out of bounds")]
    fn replay_update_out_of_bounds() {
        replay(&mut list::<i32>(), &[7, 0, 2, 1, 0, 10, 1, 1]);
    }

    #[test]
    fn replay_update() {
        let mut list = list();
        list.push(0);
        replay(&mut list, &[7, 0, 2, 1, 0, 10, 1, 1]);
        assert_eq!(list.get(0), Some(&-1));
    }

    #[test]
    fn replay_update_nested() {
        let mut list = list();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        replay(&mut list, &[8, 0, 2, 2, 0, 2, 10, 1, 100]);
        assert_eq!(list.get(0), Some(&Point::new(Runtime::new(), -1, -1, 50)));
    }

    #[test]
    fn replay_swap_remove() {
        let mut list = list();
        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));
        list.push_with(|runtime| Point::new(runtime, 2, 2, 2));

        replay(
            &mut list,
            &[1, 9, /**/ 12, 0, 2, 1, 0, 10, 6, 0, 4, 8, 4, 16, 4],
        );

        assert_eq!(list.get(0), Some(&Point::new(Runtime::new(), 2, 2, 2)));
        assert_eq!(list.get(1), None);
    }
}
