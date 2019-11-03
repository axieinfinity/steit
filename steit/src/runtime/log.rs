use std::{cell::RefCell, io, rc::Rc};

use crate::Serialize;

use super::path::Path;

#[derive(Serialize)]
#[steit(own_crate)]
pub enum EntryKind<'a, T: Serialize> {
    #[steit(tag = 0)]
    Update(#[steit(tag = 0)] &'a T),

    #[steit(tag = 1)]
    Add(#[steit(tag = 0)] &'a T),

    #[steit(tag = 2)]
    Remove,
}

#[derive(Serialize)]
#[steit(own_crate)]
pub struct Entry<'a, T: Serialize> {
    #[steit(tag = 0)]
    path: &'a Path,
    #[steit(tag = 1)]
    kind: EntryKind<'a, T>,
}

impl<'a, T: Serialize> Entry<'a, T> {
    pub fn new(path: &'a Path, kind: EntryKind<'a, T>) -> Self {
        Self { path, kind }
    }
}

#[derive(Clone)]
pub struct Logger {
    buf: Rc<RefCell<Vec<u8>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            buf: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[inline]
    pub fn log_entry(&self, entry: Entry<impl Serialize>) -> io::Result<()> {
        entry.serialize(&mut *self.buf.borrow_mut())
    }
}
