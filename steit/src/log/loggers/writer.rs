use std::io;

use crate::{
    log::{LogEntryV2, LoggerV2},
    ser_v2::SerializeV2,
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

impl LoggerV2 for WriterLogger {
    #[inline]
    fn log(&mut self, entry: LogEntryV2) -> io::Result<()> {
        let mut bytes = Vec::new();
        entry.compute_size();
        entry.serialize_nested(None, false, &mut bytes)?;
        writeln!(self.writer, "{:#?} => {:?}", entry, &bytes)
    }
}
