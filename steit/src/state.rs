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

    #[inline]
    fn replay(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        if !self.is_root() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = LogEntry::deserialize_nested(LogEntry::WIRE_TYPE, reader)?;
            let (kind, path, bytes) = entry.unpack();
            self.handle(path.into_iter(), kind, &mut Reader::new(&*bytes))?;
        }

        Ok(())
    }
}
