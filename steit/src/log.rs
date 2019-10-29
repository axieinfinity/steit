use std::io;

use crate::{path::Path, ser::Serialize};
use std::cell::RefCell;

enum EntryKind<T: Serialize> {
    Update { tag: u16, value: T },
    Add { item: T },
    Remove { tag: u16 },
}

impl<T: Serialize> EntryKind<T> {
    pub fn code(&self) -> u8 {
        match self {
            EntryKind::Update { .. } => 0,
            EntryKind::Add { .. } => 1,
            EntryKind::Remove { .. } => 2,
        }
    }
}

struct Entry<T: Serialize> {
    path: Path,
    kind: EntryKind<T>,
}

impl<T: Serialize> Serialize for Entry<T> {
    fn size(&self) -> u32 {
        1 + self.path.size()
            + match &self.kind {
                EntryKind::Update { tag, value } => tag.size() + value.size(),
                EntryKind::Add { item } => item.size(),
                EntryKind::Remove { tag } => tag.size(),
            }
    }

    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.size().serialize(writer)?;

        self.kind.code().serialize(writer)?;
        self.path.serialize(writer)?;

        match &self.kind {
            EntryKind::Update { tag, value } => {
                tag.serialize(writer)?;
                value.serialize(writer)?;
            }

            EntryKind::Add { item } => {
                item.serialize(writer)?;
            }

            EntryKind::Remove { tag } => {
                tag.serialize(writer)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Logger {
    buf: RefCell<Vec<u8>>,
}

impl Logger {
    pub fn new() -> Self {
        Self { buf: RefCell::new(Vec::new()) }
    }

    pub fn log_update<T: Serialize>(&mut self, path: Path, tag: u16, value: T) -> io::Result<()> {
        self.log_entry(Entry {
            path,
            kind: EntryKind::Update { tag, value },
        })
    }

    pub fn log_add<T: Serialize>(&mut self, path: Path, item: T) -> io::Result<()> {
        self.log_entry(Entry {
            path,
            kind: EntryKind::Add { item },
        })
    }

    pub fn log_remove<T: Serialize>(&mut self, path: Path, tag: u16) -> io::Result<()> {
        self.log_entry(Entry {
            path,
            kind: EntryKind::Remove::<T> { tag },
        })
    }

    #[inline]
    fn log_entry<T: Serialize>(&mut self, entry: Entry<T>) -> io::Result<()> {
        entry.serialize(&mut *self.buf.borrow_mut())
    }
}
