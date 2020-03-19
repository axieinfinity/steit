use std::io;

use crate::log::{LogEntry, Logger};

#[derive(Default)]
pub struct PanicLogger;

impl Logger for PanicLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        panic!("got an entry but prefer to panic! {:#?}", entry);
    }
}
