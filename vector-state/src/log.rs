use std::io;

use crate::{path::Path, ser::Serialize};

pub enum Entry<T: Serialize> {
    Update { path: Path, tag: u16, value: T },
    Add { path: Path, item: T },
    Remove { path: Path, tag: u16 },
}

impl<T: Serialize> Entry<T> {
    pub fn kind(&self) -> u8 {
        match self {
            Entry::Update { .. } => 0,
            Entry::Add { .. } => 1,
            Entry::Remove { .. } => 2,
        }
    }
}

impl<T: Serialize> Serialize for Entry<T> {
    fn size(&self) -> u32 {
        1 + match self {
            Entry::Update { path, tag, value } => path.size() + tag.size() + value.size(),
            Entry::Add { path, item } => path.size() + item.size(),
            Entry::Remove { path, tag } => path.size() + tag.size(),
        }
    }

    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        self.size().serialize(writer)?;
        self.kind().serialize(writer)?;

        match self {
            Entry::Update { path, tag, value } => {
                path.serialize(writer)?;
                tag.serialize(writer)?;
                value.serialize(writer)?;
            }

            Entry::Add { path, item } => {
                path.serialize(writer)?;
                item.serialize(writer)?;
            }

            Entry::Remove { path, tag } => {
                path.serialize(writer)?;
                tag.serialize(writer)?;
            }
        }

        Ok(())
    }
}

pub struct Logger {
    buf: Vec<u8>,
}

impl Logger {
    pub fn log_update<T: Serialize>(
        &mut self,
        path: Path,
        tag: u16,
        value: T,
    ) -> Result<(), io::Error> {
        self.log_entry(Entry::Update { path, tag, value })
    }

    pub fn log_add<T: Serialize>(&mut self, path: Path, item: T) -> Result<(), io::Error> {
        self.log_entry(Entry::Add { path, item })
    }

    pub fn log_remove<T: Serialize>(&mut self, path: Path, tag: u16) -> Result<(), io::Error> {
        self.log_entry(Entry::<T>::Remove { path, tag })
    }

    #[inline]
    fn log_entry<T: Serialize>(&mut self, entry: Entry<T>) -> Result<(), io::Error> {
        entry.serialize(&mut self.buf)
    }
}
