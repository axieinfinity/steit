use std::io;

use crate::Serialize;

mod entry;

pub use entry::LogEntry;

#[derive(Clone, Default, Debug)]
pub struct Logger {
    buf: Vec<u8>,
}

impl Logger {
    #[inline]
    pub fn new() -> Self {
        Logger { buf: Vec::new() }
    }

    #[inline]
    pub fn log_entry(&mut self, entry: LogEntry) -> io::Result<()> {
        println!("{:?}", entry.path());
        let mut buf = Vec::new();
        entry.path().serialize(&mut buf)?;
        println!("{}", entry.path().cached_size());
        println!("{:?}", buf);
        entry.compute_size();
        entry.serialize_nested_with_cached_size(None, &mut self.buf)?;
        // TODO: Remove the debug code below
        println!("=== entry: {:?}", self.buf);
        self.buf.clear();
        Ok(())
    }
}
