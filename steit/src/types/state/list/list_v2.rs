use std::{error::Error, io, ops::Deref, slice};

use crate::{
    de_v2::{DeserializeV2, Reader},
    meta::{FieldTypeMeta, HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::{RuntimeV2, SizeCache},
    ser_v2::SerializeV2,
    state_v2::StateV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

#[derive(PartialEq, Eq, Default, Hash, Debug)]
pub struct ListV2<T: StateV2> {
    items: Vec<T>,
    size_cache: SizeCache,
    runtime: RuntimeV2,
}

impl<T: StateV2> Deref for ListV2<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T: StateV2> ListV2<T> {
    #[inline]
    pub fn new(runtime: RuntimeV2) -> Self {
        Self {
            runtime,
            ..Default::default()
        }
    }

    #[inline]
    pub fn push(&mut self, mut item: T) {
        self.push_with(|runtime| {
            item.set_runtime_v2(runtime);
            item
        })
    }

    pub fn push_with(&mut self, get_item: impl FnOnce(RuntimeV2) -> T) {
        let field_number = self.items.len() as u32;

        self.runtime.pause_logger();
        let item = get_item(self.runtime.nested(field_number));
        self.runtime.unpause_logger();

        self.runtime.log_list_push(&item).unwrap();
        self.items.push(item);
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

        || -> Result<(), Box<dyn Error>> {
            let runtime = &self.runtime;
            let mut logger = runtime.logger().lock()?;
            logger.log(runtime.entry_list_pop())?;
            logger.log(runtime.entry_update_child(index as u32, &self.items[last_index]))?;
            Ok(())
        }()
        .unwrap();

        Some(self.items.swap_remove(index))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.items.iter_mut()
    }
}

impl<'a, T: StateV2> IntoIterator for &'a ListV2<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T: StateV2> IntoIterator for &'a mut ListV2<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: StateV2> HasWireType for ListV2<T> {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl<T: StateV2> SerializeV2 for ListV2<T> {
    #[inline]
    fn compute_size_v2(&self) -> u32 {
        self.items.compute_size_v2()
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

impl<T: StateV2> DeserializeV2 for ListV2<T> {
    #[inline]
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        self.items.merge_v2(reader)
    }
}

impl<T: StateV2> StateV2 for ListV2<T> {
    #[inline]
    fn with_runtime_v2(runtime: RuntimeV2) -> Self {
        Self::new(runtime)
    }

    #[inline]
    fn runtime_v2(&self) -> &RuntimeV2 {
        &self.runtime
    }

    fn set_runtime_v2(&mut self, runtime: RuntimeV2) {
        for (field_number, item) in self.items.iter_mut().enumerate() {
            item.set_runtime_v2(runtime.nested(field_number as u32))
        }

        self.runtime = runtime;
    }
}

impl<T: StateV2 + HasMeta> HasMeta for ListV2<T> {
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
        log::loggers::BufferLoggerV2,
        rt::{LoggerHandleV2, RuntimeV2},
        state_v2::StateV2,
        test_util_v2::{assert_serialize, merge, Point},
    };

    use super::ListV2;

    fn list_with_logger<T: StateV2>() -> (ListV2<T>, LoggerHandleV2<BufferLoggerV2>) {
        let (runtime, logger) = RuntimeV2::with_logger_returned(BufferLoggerV2::new());
        let list = ListV2::new(runtime);
        (list, logger)
    }

    fn list<T: StateV2>() -> ListV2<T> {
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

        let list = merge(list, &[40, 60]);

        assert_eq!(list.get(3), Some(&30));
        assert_eq!(logger.lock().unwrap().bytes(), &[]);
    }

    #[test]
    fn merge_push_new() {
        let mut list = list();

        list.push_with(|runtime| Point::new(runtime, -1, -1, -1));

        let list = merge(list, &[2, 16, 7]);

        assert_eq!(list.get(1), Some(&Point::new(RuntimeV2::new(), 0, 0, -4)));
    }

    // #[test]
    // fn replay_push_no_log() {
    //     let (list, logger) = list_with_logger();
    //     let list = replay(list, &[4, 1, 10, 1, 1]);
    //     assert_eq!(list.get(0), Some(&-1));
    //     assert_eq!(logger.lock().unwrap().bytes(), &[]);
    // }
    //
    // #[test]
    // fn replay_push_next_index() {
    //     let mut list = list();
    //
    //     list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
    //
    //     let list = merge(list, &[18, 2, 16, 7]);
    //     let list = replay(list, &[9, 1, 10, 6, 0, 3, 8, 2, 16, 10]);
    //
    //     assert_eq!(list.get(3), Some(&Point::with(Runtime::new(), -2, 1, 5)));
    // }
    //
    // #[test]
    // #[should_panic(expected = "out-of-bounds tag 0")]
    // fn replay_update_out_of_bounds() {
    //     replay(list::<i32>(), &[7, 0, 2, 1, 0, 10, 1, 1]);
    // }
    //
    // #[test]
    // #[should_panic(expected = "nonexistent item with tag 1")]
    // fn replay_update_nonexistent() {
    //     let mut list = list();
    //
    //     list.push(1);
    //     list.push(2);
    //     list.remove(1);
    //
    //     replay(list, &[7, 0, 2, 1, 1, 10, 1, 1]);
    // }
    //
    // #[test]
    // fn replay_update() {
    //     let mut list = list();
    //
    //     list.push(0);
    //
    //     let list = replay(list, &[7, 0, 2, 1, 0, 10, 1, 1]);
    //
    //     assert_eq!(list.get(0), Some(&-1));
    // }
    //
    // #[test]
    // fn replay_update_nested() {
    //     let mut list = list();
    //
    //     list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
    //
    //     let list = replay(list, &[8, 0, 2, 2, 0, 2, 10, 1, 100]);
    //
    //     assert_eq!(list.get(0), Some(&Point::with(Runtime::new(), -1, -1, 50)));
    // }
    //
    // #[test]
    // fn replay_remove() {
    //     let mut list = list();
    //
    //     list.push_with(|runtime| Point::with(runtime, -1, -1, -1));
    //     list.push_with(|runtime| Point::with(runtime, 2, 2, 2));
    //
    //     let list = replay(list, &[4, 2, 2, 1, 0]);
    //
    //     assert_eq!(list.get(0), None);
    //     assert_eq!(list.get(1), Some(&Point::with(Runtime::new(), 2, 2, 2)));
    // }
}
