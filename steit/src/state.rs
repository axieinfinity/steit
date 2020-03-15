use std::io;

use super::{
    de::Deserialize,
    log::LogEntry,
    rt::{CachedSize, Runtime},
    ser::Serialize,
    types::Bytes,
    Eof,
};

#[derive(Clone, Copy, PartialEq)]
pub enum ReplayKind {
    Update,
    Add,
    Remove,
}

#[crate::steitize(Serialize, Deserialize, own_crate, no_setters)]
#[derive(Debug)]
pub enum ReplayEntry {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0, meta_name = "flatten_path")]
        path: Vec<u16>,
        #[steit(tag = 1)]
        value: Bytes,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0, meta_name = "flatten_path")]
        path: Vec<u16>,
        #[steit(tag = 1)]
        item: Bytes,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0, meta_name = "flatten_path")]
        path: Vec<u16>,
    },
}

pub trait State: Serialize + Deserialize {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;
    fn set_runtime(&mut self, runtime: Runtime);

    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()>;

    #[inline]
    fn is_root(&self) -> bool {
        self.runtime().is_root()
    }

    #[inline]
    fn is_child(&self) -> bool {
        !self.is_root()
    }

    #[inline]
    fn handle_update(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Self::with_runtime(self.runtime().clone());
        self.merge(reader)
    }

    #[inline]
    fn replay(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if !self.is_root() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = ReplayEntry::deserialize_nested(reader)?;
            println!("{:?}", entry);

            let (path, kind, buf) = match entry {
                ReplayEntry::Update { path, value, .. } => {
                    (path, ReplayKind::Update, value.into_vec())
                }

                ReplayEntry::Add { path, item, .. } => (path, ReplayKind::Add, item.into_vec()),
                ReplayEntry::Remove { path, .. } => (path, ReplayKind::Remove, Vec::new()),
            };

            let path = &mut path.iter();
            let reader = &mut Eof::new(&*buf);

            self.handle(path, kind, reader)?;
        }

        Ok(())
    }
}

impl From<LogEntry> for ReplayEntry {
    #[inline]
    fn from(entry: LogEntry) -> Self {
        match entry {
            LogEntry::Update { path, value, .. } => ReplayEntry::Update {
                path: path.values(),
                value,
                cached_size: CachedSize::new(),
            },
            LogEntry::Add { path, item, .. } => ReplayEntry::Add {
                path: path.values(),
                item,
                cached_size: CachedSize::new(),
            },
            LogEntry::Remove { path, .. } => ReplayEntry::Remove {
                path: path.values(),
                cached_size: CachedSize::new(),
            },
        }
    }
}
