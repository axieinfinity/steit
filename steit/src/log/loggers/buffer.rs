use std::io;

use crate::{
    log::{LogEntry, Logger},
    Serialize,
};

pub struct BufferLogger {
    buf: Vec<u8>,
}

impl BufferLogger {
    #[inline]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }
}

impl Logger for BufferLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        entry.compute_size();
        entry.serialize_nested_with_cached_size(None, &mut self.buf)
    }
}
