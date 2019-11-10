use std::{cell::RefCell, io, rc::Rc};

use crate::{Runtime, Serialize};

#[crate::steitize(Serialize, own_crate, no_runtime)]
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

#[crate::steitize(Serialize, own_crate, no_runtime)]
pub struct Entry<'a, T: Serialize> {
    #[steit(tag = 0)]
    path: &'a Runtime,
    #[steit(tag = 1)]
    kind: EntryKind<'a, T>,
}

impl<'a, T: Serialize> Entry<'a, T> {
    #[inline]
    pub fn new(path: &'a Runtime, kind: EntryKind<'a, T>) -> Self {
        Self { path, kind }
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
