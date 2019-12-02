use crate::{types::Bytes, CachedSize, Runtime, Serialize};

// `path` is put in each variant and `Entry` is flattened to save serialization size.
#[crate::steitize(Serialize, own_crate)]
#[derive(Debug)]
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
            value: Bytes::with_value(value),
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn new_add(path: &'a Runtime, item: &impl Serialize) -> Self {
        LogEntry::Add {
            path,
            item: Bytes::with_value(item),
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn new_remove(path: &'a Runtime) -> Self {
        LogEntry::Remove {
            path,
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn path(&self) -> &Runtime {
        match self {
            LogEntry::Update { path, .. } => path,
            LogEntry::Add { path, .. } => path,
            LogEntry::Remove { path, .. } => path,
        }
    }
}
