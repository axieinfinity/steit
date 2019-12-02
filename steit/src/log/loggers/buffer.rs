use std::io;

use crate::log::{LogEntry, Logger};

pub struct BufferLogger {
    entries: Vec<LogEntry>,
}

impl BufferLogger {
    #[inline]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl Logger for BufferLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        self.entries.push(entry);
        Ok(())
    }
}
