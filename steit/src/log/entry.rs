use crate::{
    rt::{Node, SizeCache},
    ser::Serialize,
    steit_derive,
    types::Bytes,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogEntryKind {
    Update = 0,
    ListPush = 8,
    ListPop = 9,
    MapRemove = 12,
}

// `LogEntry` is flattened by putting `path` in each variant to save some serialization size.
#[steit_derive(Debug, Serialize, Deserialize)]
#[steit(steit_owned)]
pub enum LogEntry {
    #[steit(tag = 0, default)]
    Update {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
        #[steit(tag = 1)]
        value: Bytes,
    },
    #[steit(tag = 8)]
    ListPush {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
        #[steit(tag = 1)]
        item: Bytes,
    },
    #[steit(tag = 9)]
    ListPop {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
    },
    #[steit(tag = 12)]
    MapRemove {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
        #[steit(tag = 1)]
        key: u32,
    },
}

impl LogEntry {
    #[inline]
    pub fn new_update(path: &Node<u32>, value: &impl Serialize) -> Self {
        LogEntry::Update {
            path: path.values(),
            value: Bytes::from_value(value),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_list_push(path: &Node<u32>, item: &impl Serialize) -> Self {
        LogEntry::ListPush {
            path: path.values(),
            item: Bytes::from_value(item),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_list_pop(path: &Node<u32>) -> Self {
        LogEntry::ListPop {
            path: path.values(),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_map_remove(path: &Node<u32>, key: u32) -> Self {
        LogEntry::MapRemove {
            path: path.values(),
            key,
            size_cache: SizeCache::new(),
        }
    }

    pub fn unpack(self) -> (LogEntryKind, Vec<u32>, Vec<u8>) {
        match self {
            LogEntry::Update { path, value, .. } => (LogEntryKind::Update, path, value.into_raw()),

            LogEntry::ListPush { path, item, .. } => {
                (LogEntryKind::ListPush, path, item.into_raw())
            }

            LogEntry::ListPop { path, .. } => (LogEntryKind::ListPop, path, Vec::new()),

            LogEntry::MapRemove { path, key, .. } => {
                let mut bytes = Vec::new();
                key.serialize(&mut bytes).unwrap();
                (LogEntryKind::MapRemove, path, bytes)
            }
        }
    }
}
