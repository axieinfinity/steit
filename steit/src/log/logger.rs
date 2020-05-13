use std::io;

use super::entry::LogEntry;

pub trait Logger: Send {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;

    fn log_multi(&mut self, entries: Vec<LogEntry>) -> io::Result<()> {
        for entry in entries {
            self.log(entry)?;
        }

        Ok(())
    }
}
