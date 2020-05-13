use std::io;

use crate::{
    log::{LogEntry, Logger},
    ser::Serialize,
};

#[derive(Default)]
pub struct BufferLogger {
    entries: Vec<LogEntry>,
}

impl BufferLogger {
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
    pub fn pluck(&mut self) -> Vec<LogEntry> {
        std::mem::replace(&mut self.entries, Vec::new())
    }

    #[inline]
    pub fn pluck_bytes(&mut self) -> Vec<u8> {
        let bytes = self.bytes();
        self.clear();
        bytes
    }
}

impl Logger for BufferLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        self.entries.push(entry);
        Ok(())
    }
}
