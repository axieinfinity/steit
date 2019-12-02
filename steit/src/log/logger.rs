use std::io;

use super::entry::LogEntry;

pub trait Logger {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;
}
