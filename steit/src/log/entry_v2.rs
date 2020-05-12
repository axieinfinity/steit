use crate::{
    rt::{Node, SizeCache},
    ser_v2::SerializeV2,
    steit_derive,
    types::BytesV2,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogEntryKind {
    Update = 0,
    ListPush = 8,
    ListPop = 9,
    MapRemove = 12,
}

// `LogEntryV2` is flattened by putting `path` in each variant to save some serialization size.
#[steit_derive(Debug, Serialize, Deserialize)]
#[steit(steit_owned)]
pub enum LogEntryV2 {
    #[steit(tag = 0, default)]
    Update {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
        #[steit(tag = 1)]
        value: BytesV2,
    },
    #[steit(tag = 8)]
    ListPush {
        #[steit(tag = 0, csharp_name = "flatten_path")]
        path: Vec<u32>,
        #[steit(tag = 1)]
        item: BytesV2,
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

impl LogEntryV2 {
    #[inline]
    pub fn new_update(path: &Node<u32>, value: &impl SerializeV2) -> Self {
        LogEntryV2::Update {
            path: path.values(),
            value: BytesV2::from_value(value),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_list_push(path: &Node<u32>, item: &impl SerializeV2) -> Self {
        LogEntryV2::ListPush {
            path: path.values(),
            item: BytesV2::from_value(item),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_list_pop(path: &Node<u32>) -> Self {
        LogEntryV2::ListPop {
            path: path.values(),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_map_remove(path: &Node<u32>, key: u32) -> Self {
        LogEntryV2::MapRemove {
            path: path.values(),
            key,
            size_cache: SizeCache::new(),
        }
    }

    pub fn unpack(self) -> (LogEntryKind, Vec<u32>, Vec<u8>) {
        match self {
            LogEntryV2::Update { path, value, .. } => {
                (LogEntryKind::Update, path, value.into_raw())
            }

            LogEntryV2::ListPush { path, item, .. } => {
                (LogEntryKind::ListPush, path, item.into_raw())
            }

            LogEntryV2::ListPop { path, .. } => (LogEntryKind::ListPop, path, Vec::new()),

            LogEntryV2::MapRemove { path, key, .. } => {
                let mut bytes = Vec::new();
                key.serialize_v2(&mut bytes).unwrap();
                (LogEntryKind::MapRemove, path, bytes)
            }
        }
    }
}
