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
#[steit_derive(Clone, Debug, Serialize, Deserialize)]
#[steit(steit_owned, ctor_prefix = "empty")]
pub enum LogEntry {
    #[steit(tag = 0)]
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
    pub fn new_update(path: &Node<u32>, value: &impl Serialize) -> Self {
        LogEntry::Update {
            path: path.collect_values(),
            value: Bytes::from_value(value),
            size_cache: SizeCache::new(),
        }
    }

    pub fn new_list_push(path: &Node<u32>, item: &impl Serialize) -> Self {
        LogEntry::ListPush {
            path: path.collect_values(),
            item: Bytes::from_value(item),
            size_cache: SizeCache::new(),
        }
    }

    pub fn new_list_pop(path: &Node<u32>) -> Self {
        LogEntry::ListPop {
            path: path.collect_values(),
            size_cache: SizeCache::new(),
        }
    }

    pub fn new_map_remove(path: &Node<u32>, key: u32) -> Self {
        LogEntry::MapRemove {
            path: path.collect_values(),
            key,
            size_cache: SizeCache::new(),
        }
    }

    pub fn kind(&self) -> LogEntryKind {
        match self {
            LogEntry::Update { .. } => LogEntryKind::Update,
            LogEntry::ListPush { .. } => LogEntryKind::ListPush,
            LogEntry::ListPop { .. } => LogEntryKind::ListPop,
            LogEntry::MapRemove { .. } => LogEntryKind::MapRemove,
        }
    }
}
