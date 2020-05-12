use std::io;

use crate::{
    log::{LogEntry, Logger},
    ser::Serialize,
};

pub struct WriterLogger {
    writer: Box<dyn io::Write + Send>,
}

impl WriterLogger {
    #[inline]
    pub fn new(writer: impl io::Write + Send + 'static) -> Self {
        Self {
            writer: Box::new(writer),
        }
    }

    #[inline]
    pub fn stdout() -> Self {
        Self::new(io::stdout())
    }

    #[inline]
    pub fn stderr() -> Self {
        Self::new(io::stderr())
    }
}

impl Default for WriterLogger {
    #[inline]
    fn default() -> Self {
        Self::stdout()
    }
}

impl Logger for WriterLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        let mut bytes = Vec::new();
        entry.cache_size();
        entry.serialize_nested(None, false, &mut bytes)?;
        writeln!(self.writer, "{:#?} => {:?}", entry, &bytes)
    }
}
