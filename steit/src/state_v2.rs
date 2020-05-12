use std::io;

use super::{
    de_v2::{DeserializeV2, Reader},
    log::{LogEntryKind, LogEntryV2},
    rt::RuntimeV2,
    ser_v2::SerializeV2,
    wire_fmt::HasWireType,
};

pub trait StateV2: SerializeV2 + DeserializeV2 {
    fn with_runtime_v2(runtime: RuntimeV2) -> Self;
    fn runtime_v2(&self) -> &RuntimeV2;
    fn set_runtime_v2(&mut self, runtime: RuntimeV2);

    fn handle_v2(
        &mut self,
        path: impl Iterator<Item = u32>,
        kind: LogEntryKind,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()>;

    #[inline]
    fn is_root_v2(&self) -> bool {
        self.runtime_v2().is_root()
    }

    #[inline]
    fn is_child_v2(&self) -> bool {
        !self.is_root_v2()
    }

    #[inline]
    fn handle_update_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        *self = Self::with_runtime_v2(self.runtime_v2().clone());
        self.merge_v2(reader)
    }

    #[inline]
    fn replay(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        if !self.is_root_v2() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = LogEntryV2::deserialize_nested_v2(LogEntryV2::WIRE_TYPE, reader)?;
            let (kind, path, bytes) = entry.unpack();
            self.handle_v2(path.into_iter(), kind, &mut Reader::new(&*bytes))?;
        }

        Ok(())
    }
}
