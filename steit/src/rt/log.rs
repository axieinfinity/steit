use std::{cell::RefCell, io, rc::Rc};

use crate::Serialize;

use super::runtime::Runtime;

// `path` is put in each variant and `Entry` is flattened to save serialization size.
#[crate::steitize(Serialize, own_crate, no_runtime)]
pub enum LogEntry<'a, T: Serialize> {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        path: &'a Runtime,
        #[steit(tag = 1)]
        value: &'a T,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        path: &'a Runtime,
        #[steit(tag = 1)]
        item: &'a T,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0)]
        path: &'a Runtime,
    },
}

impl<'a, T: Serialize> LogEntry<'a, T> {
    #[inline]
    pub fn new_update(path: &'a Runtime, value: &'a T) -> Self {
        LogEntry::Update { path, value }
    }

    #[inline]
    pub fn new_add(path: &'a Runtime, item: &'a T) -> Self {
        LogEntry::Add { path, item }
    }

    #[inline]
    pub fn new_remove(path: &'a Runtime) -> Self {
        LogEntry::Remove { path }
    }

    #[inline]
    pub fn path(&self) -> &Runtime {
        match self {
            LogEntry::Update { path, .. } => path,
            LogEntry::Add { path, .. } => path,
            LogEntry::Remove { path } => path,
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
    pub fn log_entry(&self, entry: LogEntry<impl Serialize>) -> io::Result<()> {
        println!("{:?}", entry.path());
        let mut buf = Vec::new();
        println!("{}", entry.path().size());
        entry.path().serialize(&mut buf)?;
        println!("{:?}", buf);
        entry.size().serialize(&mut *self.buf.borrow_mut())?;
        entry.serialize(&mut *self.buf.borrow_mut())?;
        // TODO: Remove the debug code below
        println!("=== entry: {:?}", self.buf.borrow());
        self.buf.borrow_mut().clear();
        Ok(())
    }
}
