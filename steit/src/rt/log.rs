use std::{cell::RefCell, io, rc::Rc};

use crate::{types::bytes::Bytes, Serialize};

use super::runtime::Runtime;

// `path` is put in each variant and `Entry` is flattened to save serialization size.
#[crate::steitize(Serialize, own_crate, no_runtime)]
pub enum LogEntry<'a> {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        path: &'a Runtime,
        #[steit(tag = 1)]
        value: Bytes,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        path: &'a Runtime,
        #[steit(tag = 1)]
        item: Bytes,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0)]
        path: &'a Runtime,
    },
}

impl<'a> LogEntry<'a> {
    #[inline]
    pub fn new_update(path: &'a Runtime, value: &impl Serialize) -> Self {
        LogEntry::Update {
            path,
            value: Bytes::new(value),
        }
    }

    #[inline]
    pub fn new_add(path: &'a Runtime, item: &impl Serialize) -> Self {
        LogEntry::Add {
            path,
            item: Bytes::new(item),
        }
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
    pub fn log_entry(&self, entry: LogEntry) -> io::Result<()> {
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
