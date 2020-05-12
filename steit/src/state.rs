use std::io;

use super::{
    de::{Deserialize, Reader},
    log::{LogEntry, LogEntryKind},
    rt::Runtime,
    ser::Serialize,
    wire_fmt::HasWireType,
};

pub trait State: Serialize + Deserialize {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;
    fn set_runtime(&mut self, runtime: Runtime);

    fn handle(
        &mut self,
        path: impl Iterator<Item = u32>,
        kind: LogEntryKind,
        key: Option<u32>,
        reader: &mut Reader<impl io::Read>,
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
    fn handle_update(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        *self = Self::with_runtime(self.runtime().clone());
        self.merge(reader)
    }

    fn replay(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        if !self.is_root() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = LogEntry::deserialize_nested(LogEntry::WIRE_TYPE, reader)?;

            let (kind, path, key, bytes) = unpack_log_entry(entry);
            let path = path.into_iter();
            let bytes = bytes.unwrap_or_default();
            let reader = &mut Reader::new(&*bytes);

            self.handle(path, kind, key, reader)?;
        }

        Ok(())
    }
}

fn unpack_log_entry(entry: LogEntry) -> (LogEntryKind, Vec<u32>, Option<u32>, Option<Vec<u8>>) {
    match entry {
        LogEntry::Update { path, value, .. } => {
            (LogEntryKind::Update, path, None, Some(value.into_raw()))
        }

        LogEntry::ListPush { path, item, .. } => {
            (LogEntryKind::ListPush, path, None, Some(item.into_raw()))
        }

        LogEntry::ListPop { path, .. } => (LogEntryKind::ListPop, path, None, None),
        LogEntry::MapRemove { path, key, .. } => (LogEntryKind::MapRemove, path, Some(key), None),
    }
}
