use crate::{
    rt::{Node, SizeCache},
    ser_v2::SerializeV2,
    steit_derive,
    types::BytesV2,
};

// `LogEntryV2` is flattened by putting `path` in each variant to save some serialization size.
#[steit_derive(Debug, Serialize, Deserialize)]
#[steit(steit_owned)]
pub enum LogEntryV2 {
    #[steit(tag = 1, default)]
    Update {
        #[steit(tag = 1)]
        path: Vec<u32>,
        #[steit(tag = 2)]
        value: BytesV2,
    },
    #[steit(tag = 8)]
    ListPush {
        #[steit(tag = 1)]
        path: Vec<u32>,
        #[steit(tag = 2)]
        item: BytesV2,
    },
    #[steit(tag = 9)]
    ListPop {
        #[steit(tag = 1)]
        path: Vec<u32>,
    },
    #[steit(tag = 12)]
    MapInsert {
        #[steit(tag = 1)]
        path: Vec<u32>,
        #[steit(tag = 2)]
        key: BytesV2,
        #[steit(tag = 3)]
        value: BytesV2,
    },
    #[steit(tag = 13)]
    MapRemove {
        #[steit(tag = 1)]
        path: Vec<u32>,
        #[steit(tag = 2)]
        key: BytesV2,
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
    pub fn new_map_insert(
        path: &Node<u32>,
        key: &impl SerializeV2,
        value: &impl SerializeV2,
    ) -> Self {
        LogEntryV2::MapInsert {
            path: path.values(),
            key: BytesV2::from_value(key),
            value: BytesV2::from_value(value),
            size_cache: SizeCache::new(),
        }
    }

    #[inline]
    pub fn new_map_remove(path: &Node<u32>, key: &impl SerializeV2) -> Self {
        LogEntryV2::MapRemove {
            path: path.values(),
            key: BytesV2::from_value(key),
            size_cache: SizeCache::new(),
        }
    }

    pub fn path(&self) -> &[u32] {
        match self {
            LogEntryV2::Update { path, .. }
            | LogEntryV2::ListPush { path, .. }
            | LogEntryV2::ListPop { path, .. }
            | LogEntryV2::MapInsert { path, .. }
            | LogEntryV2::MapRemove { path, .. } => path,
        }
    }
}
