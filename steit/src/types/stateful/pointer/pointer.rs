use std::{borrow::BorrowMut, io, rc::Rc};

use crate::{
    gen::{HasFieldType, HasMeta, Meta},
    wire_type,
    wire_type::{WireType, WIRE_TYPE_SIZED},
    CachedSize, Deserialize, Eof, Merge, ReplayKind, Runtime, Serialize, State,
};

#[derive(Default, Debug)]
pub struct Pointer<T: State> {
    item: Rc<Option<T>>,
    cached_size: CachedSize,
    runtime: Runtime,
}

impl<T: State> Pointer<T> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            item: Rc::new(None),
            cached_size: CachedSize::new(),
            runtime,
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut Option<T>> {
        Rc::get_mut(self.item.borrow_mut())
    }

    pub fn set(&mut self, item: T) {
        self.runtime.log_update(0, &item).unwrap();
        self.item = Rc::new(Some(item));
    }

    #[inline]
    pub fn set_with(&mut self, f: impl FnOnce(Runtime) -> T) {
        self.runtime.pause_logger();
        let item = f(self.runtime.nested(0));
        self.runtime.unpause_logger();
        self.item = Rc::new(Some(item))
    }
}

impl<T: State> WireType for Pointer<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: State> Serialize for Pointer<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.item.compute_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.item.serialize_with_cached_size(writer)
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.cached_size.get()
    }
}

impl<T: State> Merge for Pointer<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let key = u32::deserialize(reader)?;
            let (tag, wire_type) = wire_type::split_key(key);

            if wire_type == T::WIRE_TYPE {
                let value = self.get_mut().unwrap();

                if let Some(value) = value {
                    value.merge_nested(reader)?;
                } else {
                    self.item = Rc::new(Some(T::with_runtime(self.runtime.nested(tag))));
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

impl<T: State> State for Pointer<T> {
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

            if tag == 0 {
                if kind == &ReplayKind::Update && path.peek().is_none() {
                    let mut value = T::with_runtime(self.runtime().nested(tag));
                    value.merge(reader)?;
                    self.item = Rc::new(Some(value));
                    Ok(())
                } else if let Some(value) = self.get_mut().unwrap() {
                    if kind == &ReplayKind::Remove && path.peek().is_none() {
                        self.item = Rc::new(None);
                        Ok(())
                    } else {
                        value.handle(path, kind, reader)
                    }
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("nonexistent value"),
                    ))
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
                    "`add` and `remove` are not supported on the current `Pointer` but its items",
                )),
            }
        }
    }
}

impl<T: State + HasFieldType> HasMeta for Pointer<T> {
    const META: &'static Meta = &Meta::Rc(T::FIELD_TYPE);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        log::loggers::BufferLogger,
        test_utils::{assert_serialize, assert_size, merge, replay, Point},
        Runtime, State,
    };

    use super::Pointer;

    fn pointer_with_logger<T: State>() -> (Pointer<T>, Rc<RefCell<BufferLogger>>) {
        let logger = Rc::new(RefCell::new(BufferLogger::new()));
        let pointer = Pointer::new(Runtime::with_logger(Box::new(logger.clone())));
        (pointer, logger)
    }

    fn pointer<T: State>() -> Pointer<T> {
        pointer_with_logger().0
    }

    #[test]
    fn size() {
        let pointer: Pointer<i32> = pointer();

        assert_size(pointer, 0);
    }

    #[test]
    fn set_and_get() {
        let mut pointer: Pointer<i32> = pointer();
        pointer.set(8);

        assert_eq!(pointer.get_mut(), Some(&mut Some(8)));
    }

    #[test]
    fn set_and_check_log() {
        let (mut pointer, logger): (Pointer<i32>, Rc<RefCell<BufferLogger>>) =
            pointer_with_logger();
        pointer.set(8);

        assert_eq!(logger.borrow().bytes(), &[7, 0, 2, 1, 0, 10, 1, 16]);
    }

    #[test]
    fn serialize_01() {
        let mut pointer: Pointer<i32> = pointer();
        pointer.set(8);

        assert_serialize(pointer, &[16]);
    }

    #[test]
    fn serialize_02() {
        let mut pointer: Pointer<Point> = pointer();
        pointer.set_with(|runtime| {
            let point = Point::with(runtime, -1, -1, -1);
            point
        });

        assert_serialize(pointer, &[6, 0, 1, 8, 1, 16, 1]);
    }

    #[test]
    fn merge_no_log() {
        let (mut pointer, logger): (Pointer<i32>, Rc<RefCell<BufferLogger>>) =
            pointer_with_logger();
        pointer.set(8);

        logger.borrow_mut().clear();

        let mut pointer = merge(pointer, &[40, 60]);

        assert_eq!(pointer.get_mut().unwrap(), &mut Some(30));
        assert_eq!(logger.borrow().bytes(), &[]);
    }

    #[test]
    fn merge_update_nested() {
        let mut pointer = pointer();
        pointer.set_with(|runtime| Point::with(runtime, -1, -1, -1));

        let mut pointer = merge(pointer, &[2, 2, 8, 5]);

        assert_eq!(
            pointer.get_mut().unwrap(),
            &mut Some(Point::with(Runtime::new(), -1, -3, -1))
        );
    }

    #[test]
    fn replay_update() {
        let pointer = pointer();

        let mut pointer = replay(pointer, &[7, 0, 2, 1, 0, 10, 1, 16]);

        assert_eq!(pointer.get_mut().unwrap(), &mut Some(8));
    }

    #[test]
    fn replay_update_nested() {
        let mut pointer = pointer();
        pointer.set_with(|runtime| Point::with(runtime, -1, -1, -1));

        let mut pointer = replay(pointer, &[8, 0, 2, 2, 0, 2, 10, 1, 100]);

        assert_eq!(
            pointer.get_mut().unwrap(),
            &mut Some(Point::with(Runtime::new(), -1, -1, 50))
        );
    }

    #[test]
    fn replay_remove() {
        let mut pointer = pointer();
        pointer.set_with(|runtime| Point::with(runtime, -1, -1, -1));

        let mut pointer = replay(pointer, &[4, 2, 2, 1, 0]);

        assert_eq!(pointer.get_mut().unwrap(), &mut None);
    }
}
