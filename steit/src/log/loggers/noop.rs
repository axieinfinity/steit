use std::io;

use crate::log::{LogEntry, Logger};

#[derive(Default)]
pub struct NoopLogger;

impl Logger for NoopLogger {
    #[inline]
    fn log(&mut self, _entry: LogEntry) -> io::Result<()> {
        Ok(())
    }
}
