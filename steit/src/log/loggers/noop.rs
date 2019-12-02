use std::io;

use crate::log::{LogEntry, Logger};

pub struct NoopLogger;

impl NoopLogger {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Logger for NoopLogger {
    #[inline]
    fn log(&mut self, _entry: LogEntry) -> io::Result<()> {
        Ok(())
    }
}
