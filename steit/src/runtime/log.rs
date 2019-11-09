use std::{cell::RefCell, io, rc::Rc};

use crate::{Runtime, Serialize};

#[crate::serialize(own_crate)]
pub enum EntryKind<'a, T: Serialize> {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        value: &'a T,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        item: &'a T,
    },
    #[steit(tag = 2)]
    Remove,
}

#[crate::serialize(own_crate)]
pub struct Entry<'a, T: Serialize> {
    #[steit(tag = 0)]
    path: &'a Runtime,
    #[steit(tag = 1)]
    kind: EntryKind<'a, T>,
}

impl<'a, T: Serialize> Entry<'a, T> {
    #[inline]
    pub fn new_update(path: &'a Runtime, value: &'a T) -> Self {
        Self {
            runtime: Runtime::new(),
            path,
            kind: EntryKind::Update {
                runtime: Runtime::new(),
                value,
            },
        }
    }

    #[inline]
    pub fn new_add(path: &'a Runtime, item: &'a T) -> Self {
        Self {
            runtime: Runtime::new(),
            path,
            kind: EntryKind::Add {
                runtime: Runtime::new(),
                item,
            },
        }
    }

    #[inline]
    pub fn new_remove(path: &'a Runtime) -> Self {
        Self {
            runtime: Runtime::new(),
            path,
            kind: EntryKind::Remove {
                runtime: Runtime::new(),
            },
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Logger {
    buf: Rc<RefCell<Vec<u8>>>,
}

impl Logger {
    #[inline]
    pub fn new() -> Self {
        Logger {
            buf: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[inline]
    pub fn log_entry(&self, entry: Entry<impl Serialize>) -> io::Result<()> {
        entry.serialize(&mut *self.buf.borrow_mut())?;
        // TODO: Remove the debug code below
        println!("=== entry: {:?}", self.buf.borrow());
        self.buf.borrow_mut().clear();
        Ok(())
    }
}
