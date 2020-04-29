use std::io;

use crate::{
    log::{LogEntryV2, LoggerV2},
    ser_v2::SerializeV2,
};

#[derive(Default)]
pub struct BufferLoggerV2 {
    entries: Vec<LogEntryV2>,
}

impl BufferLoggerV2 {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for entry in &self.entries {
            entry.cache_size();
            entry.serialize_nested(None, false, &mut bytes).unwrap();
        }

        bytes
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    #[inline]
    pub fn pluck(&mut self) -> Vec<u8> {
        let bytes = self.bytes();
        self.clear();
        bytes
    }
}

impl LoggerV2 for BufferLoggerV2 {
    #[inline]
    fn log(&mut self, entry: LogEntryV2) -> io::Result<()> {
        self.entries.push(entry);
        Ok(())
    }
}
