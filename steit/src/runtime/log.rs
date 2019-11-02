use std::{cell::RefCell, io, rc::Rc};

use crate::Serialize;

use super::path::Path;

pub enum EntryKind<'a, T: Serialize> {
    Update { tag: u16, value: &'a T },
    Add { item: &'a T },
    Remove { tag: u16 },
}

impl<'a, T: Serialize> EntryKind<'a, T> {
    pub fn code(&self) -> u8 {
        match self {
            EntryKind::Update { .. } => 0,
            EntryKind::Add { .. } => 1,
            EntryKind::Remove { .. } => 2,
        }
    }
}

pub struct Entry<'a, T: Serialize> {
    path: &'a Path,
    kind: EntryKind<'a, T>,
}

impl<'a, T: Serialize> Entry<'a, T> {
    pub fn new(path: &'a Path, kind: EntryKind<'a, T>) -> Self {
        Self { path, kind }
    }
}

impl<'a, T: Serialize> Serialize for Entry<'a, T> {
    fn size(&self) -> u32 {
        1 + self.path.size().size()
            + self.path.size()
            + match self.kind {
                EntryKind::Update { tag, value } => tag.size() + value.size(),
                EntryKind::Add { item } => item.size(),
                EntryKind::Remove { tag } => tag.size(),
            }
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.size().serialize(writer)?;

        self.kind.code().serialize(writer)?;
        self.path.serialize(writer)?;

        match self.kind {
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
    buf: Rc<RefCell<Vec<u8>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            buf: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[inline]
    pub fn log_entry(&self, entry: Entry<impl Serialize>) -> io::Result<()> {
        entry.serialize(&mut *self.buf.borrow_mut())
    }
}
