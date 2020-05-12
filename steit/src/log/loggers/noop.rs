use std::io;

use crate::log::{LogEntryV2, LoggerV2};

#[derive(Default)]
pub struct NoopLogger;

impl NoopLogger {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl LoggerV2 for NoopLogger {
    #[inline]
    fn log(&mut self, _entry: LogEntryV2) -> io::Result<()> {
        Ok(())
    }
}
