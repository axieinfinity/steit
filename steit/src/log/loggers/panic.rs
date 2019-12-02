use std::io;

use crate::log::{LogEntry, Logger};

pub struct PanicLogger;

impl PanicLogger {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Logger for PanicLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        panic!("got an entry but prefer to panic! {:#?}", entry);
    }
}
