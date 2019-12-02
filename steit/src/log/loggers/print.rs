use std::io;

use crate::{
    log::{LogEntry, Logger},
    Serialize,
};

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
        let mut bytes = Vec::new();
        entry.compute_size();
        entry.serialize_nested_with_cached_size(None, &mut bytes)?;
        writeln!(self.writer, "{:#?} => {:?}", entry, &bytes)
    }
}
