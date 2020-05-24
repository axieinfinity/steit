use std::io;

use crate::log::{LogEntry, Logger};

#[derive(Default)]
pub struct NoopLogger;

impl NoopLogger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Logger for NoopLogger {
    fn log(&mut self, _entry: LogEntry) -> io::Result<()> {
        Ok(())
    }
}
