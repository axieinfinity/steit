use std::io;

use crate::{
    log::{LogEntry, Logger},
    ser::Serialize,
};

pub struct WriterLogger {
    writer: Box<dyn io::Write + Send>,
}

impl WriterLogger {
    pub fn new(writer: impl io::Write + Send + 'static) -> Self {
        Self {
            writer: Box::new(writer),
        }
    }

    pub fn stdout() -> Self {
        Self::new(io::stdout())
    }

    pub fn stderr() -> Self {
        Self::new(io::stderr())
    }
}

impl Default for WriterLogger {
    fn default() -> Self {
        Self::stdout()
    }
}

impl Logger for WriterLogger {
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        let mut bytes = Vec::new();
        entry.cache_size();
        entry.serialize_nested(None, false, &mut bytes)?;
        writeln!(self.writer, "{:#?} => {:?}", entry, &bytes)
    }
}
