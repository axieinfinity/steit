use std::io;

use crate::{
    wire_type::{self, WireType, WIRE_TYPE_SIZED},
    Deserialize, Eof, Merge, ReplayKind, Runtime, Runtimed, Serialize, State,
};

use super::iter::{Iter, IterMut};

#[derive(Default, Debug)]
pub struct List<T: State> {
    runtime: Runtime,
    items: Vec<Option<T>>,
}

impl<T: State> List<T> {
    #[inline]
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            items: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.runtime.log_add(&item).unwrap();
        self.items.push(Some(item));
        self.runtime.clear_cached_size();
    }

    #[inline]
    pub fn push_with(&mut self, f: impl FnOnce(Runtime) -> T) {
        let tag = self.items.len();

        assert!(
            tag <= u16::max_value() as usize,
            "`List` indices must be within `u16`"
        );

        let item = f(self.runtime.nested(tag as u16));
        self.push(item);
    }

    #[inline]
    pub fn remove(&mut self, tag: u16) {
        match self.items.get_mut(tag as usize) {
            Some(item) => {
                self.runtime.log_remove(tag).unwrap();
                *item = None;
                self.runtime.clear_cached_size();
            }
            None => (),
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.items.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self.items.iter_mut())
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

impl<T: State> Runtimed for List<T> {
    #[inline]
    fn with_runtime(runtime: Runtime) -> Self {
        Self::new(runtime)
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        &self.runtime
    }
}

impl<T: State> Serialize for List<T> {
    #[inline]
    fn size(&self) -> u32 {
        self.runtime.get_or_set_cached_size_from(|| {
            assert!(
                self.items.len() <= u16::max_value() as usize + 1,
                "`List` indices must be within `u16`"
            );

            let mut size = 0;

            for (tag, item) in self.items.iter().enumerate() {
                if let Some(item) = item {
                    size += item.size_nested(tag as u16);
                }
            }

            size
        })
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        assert!(
            self.items.len() <= u16::max_value() as usize + 1,
            "`List` indices must be within `u16`"
        );

        for (tag, item) in self.items.iter().enumerate() {
            if let Some(item) = item {
                item.serialize_nested(tag as u16, writer)?;
            }
        }

        Ok(())
    }
}

impl<T: State> Merge for List<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        let mut changed = false;

        while !reader.eof()? {
            let key = u32::deserialize(reader)?;
            let (tag, wire_type) = wire_type::split_key(key);

            if wire_type == T::WIRE_TYPE {
                let item = T::deserialize_nested(reader)?;

                if tag as usize >= self.items.len() {
                    self.items.resize_with(tag as usize + 1, || None);
                }

                self.items[tag as usize] = Some(item);
                changed = true;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unexpected wire type {}", wire_type),
                ));
            }
        }

        if changed {
            self.runtime.clear_cached_size();
        }

        Ok(())
    }
}

impl<T: State> State for List<T> {
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
                        self.runtime.clear_cached_size();
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
                    self.runtime.clear_cached_size();
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

#[cfg(test)]
mod test {
    use crate::{Deserialize, Eof, Merge, Runtime, State};

    use super::List;

    #[test]
    fn merge() {
        let mut list = List::<u16>::new(Runtime::new());

        list.merge(&mut Eof::new([8, 1, 24, 3, 0, 0].as_ref()))
            .unwrap();

        let items: Vec<_> = list.iter().map(|item| *item).collect();

        assert_eq!(&items, &[0, 1, 3]);
    }

    #[test]
    fn replay() {
        let mut list =
            List::<u16>::deserialize(&mut Eof::new([8, 1, 24, 3, 0, 0].as_ref())).unwrap();

        let items: Vec<_> = list.iter().map(|item| *item).collect();

        assert_eq!(&items, &[0, 1, 3]);

        list.replay(&mut Eof::new([7, 0, 2, 1, 1, 10, 1, 2].as_ref()))
            .unwrap();

        let items: Vec<_> = list.iter().map(|item| *item).collect();

        assert_eq!(&items, &[0, 2, 3]);
    }
}
