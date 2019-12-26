use std::{cell::RefCell, io, rc::Rc};

use super::entry::LogEntry;

pub trait Logger {
    fn log(&mut self, entry: LogEntry) -> io::Result<()>;
}

impl<T: Logger> Logger for Rc<RefCell<T>> {
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        self.borrow_mut().log(entry)
    }
}
