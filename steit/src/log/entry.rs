use std::rc::Rc;

use crate::{node::Node, types::Bytes, CachedSize, Serialize};

// `path` is put in each variant and `Entry` is flattened to save serialization size.
#[crate::steitize(Serialize, own_crate)]
#[derive(Debug)]
pub enum LogEntry {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        path: Rc<Node<u16>>,
        #[steit(tag = 1)]
        value: Bytes,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        path: Rc<Node<u16>>,
        #[steit(tag = 1)]
        item: Bytes,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0)]
        path: Rc<Node<u16>>,
    },
}

impl LogEntry {
    #[inline]
    pub fn new_update(path: Rc<Node<u16>>, value: &impl Serialize) -> Self {
        LogEntry::Update {
            path,
            value: Bytes::with_value(value),
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn new_add(path: Rc<Node<u16>>, item: &impl Serialize) -> Self {
        LogEntry::Add {
            path,
            item: Bytes::with_value(item),
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn new_remove(path: Rc<Node<u16>>) -> Self {
        LogEntry::Remove {
            path,
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn path(&self) -> &Rc<Node<u16>> {
        match self {
            LogEntry::Update { path, .. } => path,
            LogEntry::Add { path, .. } => path,
            LogEntry::Remove { path, .. } => path,
        }
    }
}
