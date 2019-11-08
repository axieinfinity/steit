use std::{cell::RefCell, io, rc::Rc};

use crate::{Runtime2, Serialize2};

#[crate::serialize(own_crate)]
pub enum EntryKind<'a, T: Serialize2> {
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
pub struct Entry<'a, T: Serialize2> {
    #[steit(tag = 0)]
    path: &'a Runtime2,
    #[steit(tag = 1)]
    kind: EntryKind<'a, T>,
}

impl<'a, T: Serialize2> Entry<'a, T> {
    #[inline]
    pub fn new_update(path: &'a Runtime2, value: &'a T) -> Self {
        Self {
            runtime: Runtime2::new(),
            path,
            kind: EntryKind::Update {
                runtime: Runtime2::new(),
                value,
            },
        }
    }

    #[inline]
    pub fn new_add(path: &'a Runtime2, item: &'a T) -> Self {
        Self {
            runtime: Runtime2::new(),
            path,
            kind: EntryKind::Add {
                runtime: Runtime2::new(),
                item,
            },
        }
    }

    #[inline]
    pub fn new_remove(path: &'a Runtime2) -> Self {
        Self {
            runtime: Runtime2::new(),
            path,
            kind: EntryKind::Remove {
                runtime: Runtime2::new(),
            },
        }
    }
}

#[derive(Clone, Debug)]
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
    pub fn log_entry(&self, entry: Entry<impl Serialize2>) -> io::Result<()> {
        entry.serialize(&mut *self.buf.borrow_mut())?;
        // TODO: Remove the debug code below
        println!("=== entry: {:?}", self.buf.borrow());
        self.buf.borrow_mut().clear();
        Ok(())
    }
}

impl Default for Logger {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
