use std::io;

use super::entry_v2::LogEntryV2;

pub trait LoggerV2: Send {
    fn log(&mut self, entry: LogEntryV2) -> io::Result<()>;
}
