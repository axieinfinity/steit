use std::io;

use super::entry::LogEntry;

pub trait Logger: Send {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;
}
