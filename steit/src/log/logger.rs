use std::{
    io,
    sync::{Arc, Mutex},
};

use super::entry::LogEntry;

pub trait Logger: Send + Sync {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;
}

impl<T: Logger> Logger for Arc<Mutex<T>> {
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        self.lock().unwrap().log(entry)
    }
}
