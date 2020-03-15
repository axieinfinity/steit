use std::io;

use crate::{
    log::{LogEntry, Logger},
    Serialize,
};

pub struct BufferLogger {
    log: Vec<LogEntry>,
}

impl BufferLogger {
    #[inline]
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for entry in &self.log {
            entry.compute_size();
            entry
                .serialize_nested_with_cached_size(None, &mut bytes)
                .unwrap();
        }

        bytes
    }

    #[inline]
    pub fn clear(&mut self) {
        self.log.clear();
    }

    #[inline]
    pub fn pluck(&mut self) -> Vec<u8> {
        let bytes = self.bytes();
        self.clear();
        bytes
    }

    #[inline]
    pub fn pluck_log(&mut self) -> Vec<LogEntry> {
        std::mem::replace(&mut self.log, Vec::new())
    }
}

impl Logger for BufferLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        self.log.push(entry);
        Ok(())
    }
}
