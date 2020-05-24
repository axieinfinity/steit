use std::{
    fmt,
    hash::{Hash, Hasher},
    io,
    sync::{Arc, Mutex},
};

use crate::{
    log::{loggers::WriterLogger, LogEntry, Logger},
    ser::Serialize,
};

use super::{
    logger::{LoggerHandle, PausableLogger, RuntimeLogger},
    node::Node,
};

#[derive(Clone)]
pub struct Runtime {
    logger: Arc<Mutex<dyn PausableLogger>>,
    path: Arc<Node<u32>>,
}

macro_rules! impl_entry {
    ($entry:ident, $new_entry:ident $(, $param_name:ident : $param_type:ty )* $(,)?) => {
        pub fn $entry(&self $(, $param_name: $param_type )*) -> LogEntry {
            LogEntry::$new_entry(&self.path $(, $param_name )*)
        }
    };
}

macro_rules! impl_log {
    ($log:ident, $entry:ident $(, $param_name:ident : $param_type:ty )* $(,)?) => {
        pub fn $log(&self $(, $param_name: $param_type)*) -> io::Result<()> {
            self.log(self.$entry($($param_name ),*))
        }
    };
}

impl Runtime {
    pub fn new() -> Self {
        Self::with_logger(WriterLogger::stdout())
    }

    pub fn with_logger_returned<T: Logger + 'static>(logger: T) -> (Self, LoggerHandle<T>) {
        let logger = Arc::new(Mutex::new(RuntimeLogger::new(logger)));

        (
            Self {
                logger: logger.clone(),
                path: Arc::new(Node::Root),
            },
            logger,
        )
    }

    pub fn with_logger<T: Logger + 'static>(logger: T) -> Self {
        Self::with_logger_returned(logger).0
    }

    pub fn nested(&self, field_number: u32) -> Self {
        Self {
            logger: self.logger.clone(),
            path: Arc::new(Node::child(&self.path, field_number)),
        }
    }

    pub fn parent(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.parent(),
        }
    }

    pub fn path(&self) -> &Arc<Node<u32>> {
        &self.path
    }

    pub fn is_root(&self) -> bool {
        match &*self.path {
            Node::Root { .. } => true,
            Node::Child { .. } => false,
        }
    }

    pub fn is_child(&self) -> bool {
        !self.is_root()
    }

    pub fn get_field_number(&self) -> Option<u32> {
        self.path.get_value().copied()
    }

    pub fn field_number(&self) -> u32 {
        *self.path.value()
    }

    pub fn logger(&self) -> &Arc<Mutex<dyn PausableLogger>> {
        &self.logger
    }

    pub fn pause_logger(&self) -> u32 {
        self.logger.lock().unwrap().pause()
    }

    pub fn unpause_logger(&self) -> u32 {
        self.logger.lock().unwrap().unpause()
    }

    pub fn log(&self, entry: LogEntry) -> io::Result<()> {
        self.logger.lock().unwrap().log(entry)
    }

    pub fn log_multi(&self, entries: Vec<LogEntry>) -> io::Result<()> {
        self.logger.lock().unwrap().log_multi(entries)
    }

    impl_entry!(entry_update, new_update, value: &impl Serialize);
    impl_entry!(entry_list_push, new_list_push, item: &impl Serialize);
    impl_entry!(entry_list_pop, new_list_pop);
    impl_entry!(entry_map_remove, new_map_remove, key: u32);

    pub fn entry_update_child(&self, field_number: u32, value: &impl Serialize) -> LogEntry {
        LogEntry::new_update(&Node::child(&self.path, field_number), value)
    }

    impl_log!(log_update, entry_update, value: &impl Serialize);
    impl_log!(
        log_update_child,
        entry_update_child,
        field_number: u32,
        value: &impl Serialize,
    );
    impl_log!(log_list_push, entry_list_push, item: &impl Serialize);
    impl_log!(log_list_pop, entry_list_pop);
    impl_log!(log_map_remove, entry_map_remove, key: u32);
}

impl PartialEq for Runtime {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Runtime {}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for Runtime {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Runtime")
            .field("logger", &"<logger>")
            .field("path", &*self.path)
            .finish()
    }
}
