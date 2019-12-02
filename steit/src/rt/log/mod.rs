use std::io::{self, Write};

use crate::Serialize;

mod entry;

pub use entry::LogEntry;

pub trait Logger {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;
}

pub struct PrintLogger {
    writer: Box<dyn io::Write>,
}

impl PrintLogger {
    #[inline]
    pub fn new(writer: Box<dyn io::Write>) -> Self {
        Self { writer }
    }

    #[inline]
    pub fn with_stdout() -> Self {
        Self::new(Box::new(io::stdout()))
    }

    #[inline]
    pub fn with_stderr() -> Self {
        Self::new(Box::new(io::stderr()))
    }
}

impl Logger for PrintLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        writeln!(self.writer, "{:?}", entry)
    }
}

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
