pub enum ReplayKind {
    Update,
    Add,
    Remove,
}

#[crate::steitize(Deserialize, own_crate, no_runtime)]
#[derive(Debug)]
pub enum ReplayEntry {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        path: Vec<u16>,
        #[steit(tag = 1)]
        value: Vec<u8>,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        path: Vec<u16>,
        #[steit(tag = 1)]
        item: Vec<u8>,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0)]
        path: Vec<u16>,
    },
}

impl ReplayEntry {
    pub fn kind(&self) -> ReplayKind {
        match self {
            ReplayEntry::Update { .. } => ReplayKind::Update,
            ReplayEntry::Add { .. } => ReplayKind::Add,
            ReplayEntry::Remove { .. } => ReplayKind::Remove,
        }
    }
}
