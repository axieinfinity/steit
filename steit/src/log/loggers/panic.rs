use std::io;

use crate::log::{LogEntryV2, LoggerV2};

#[derive(Default)]
pub struct PanicLogger;

impl PanicLogger {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl LoggerV2 for PanicLogger {
    #[inline]
    fn log(&mut self, entry: LogEntryV2) -> io::Result<()> {
        panic!("got an entry but prefer to panic! {:#?}", entry);
    }
}
