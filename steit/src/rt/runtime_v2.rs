use std::{
    fmt,
    hash::{Hash, Hasher},
    io,
    sync::{Arc, Mutex},
};

use crate::{
    log::{loggers::WriterLogger, LogEntryV2, LoggerV2},
    ser_v2::SerializeV2,
};

use super::{
    logger_v2::{LoggerHandleV2, PausableLoggerV2, RuntimeLoggerV2},
    node::Node,
};

#[derive(Clone)]
pub struct RuntimeV2 {
    logger: Arc<Mutex<dyn PausableLoggerV2>>,
    path: Arc<Node<u32>>,
}

macro_rules! impl_entry {
    ($entry:ident, $new_entry:ident $(, $param_name:ident : $param_type:ty )* $(,)?) => {
        #[inline]
        pub fn $entry(&self $(, $param_name: $param_type )*) -> LogEntryV2 {
            LogEntryV2::$new_entry(&self.path $(, $param_name )*)
        }
    };
}

macro_rules! impl_log {
    ($log:ident, $entry:ident $(, $param_name:ident : $param_type:ty )* $(,)?) => {
        #[inline]
        pub fn $log(&self $(, $param_name: $param_type)*) -> io::Result<()> {
            self.log(self.$entry($($param_name ),*))
        }
    };
}

impl RuntimeV2 {
    #[inline]
    pub fn new() -> Self {
        Self::with_logger(WriterLogger::stdout())
    }

    #[inline]
    pub fn with_logger_returned<T: LoggerV2 + 'static>(logger: T) -> (Self, LoggerHandleV2<T>) {
        let logger = Arc::new(Mutex::new(RuntimeLoggerV2::new(logger)));

        (
            Self {
                logger: logger.clone(),
                path: Arc::new(Node::Root),
            },
            logger,
        )
    }

    #[inline]
    pub fn with_logger<T: LoggerV2 + 'static>(logger: T) -> Self {
        Self::with_logger_returned(logger).0
    }

    #[inline]
    pub fn nested(&self, field_number: u32) -> Self {
        Self {
            logger: self.logger.clone(),
            path: Arc::new(Node::child(&self.path, field_number)),
        }
    }

    #[inline]
    pub fn parent(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.parent().expect("expected a parent `RuntimeV2`"),
        }
    }

    #[inline]
    pub fn path(&self) -> &Arc<Node<u32>> {
        &self.path
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        match &*self.path {
            Node::Root { .. } => true,
            Node::Child { .. } => false,
        }
    }

    #[inline]
    pub fn is_child(&self) -> bool {
        !self.is_root()
    }

    #[inline]
    pub fn logger(&self) -> &Arc<Mutex<dyn PausableLoggerV2>> {
        &self.logger
    }

    #[inline]
    pub fn pause_logger(&self) -> u32 {
        self.logger.lock().unwrap().pause()
    }

    #[inline]
    pub fn unpause_logger(&self) -> u32 {
        self.logger.lock().unwrap().unpause()
    }

    #[inline]
    pub fn log(&self, entry: LogEntryV2) -> io::Result<()> {
        self.logger.lock().unwrap().log(entry)
    }

    impl_entry!(entry_update, new_update, value: &impl SerializeV2);
    impl_entry!(entry_list_push, new_list_push, item: &impl SerializeV2);
    impl_entry!(entry_list_pop, new_list_pop);

    #[inline]
    pub fn entry_update_child(&self, field_number: u32, value: &impl SerializeV2) -> LogEntryV2 {
        LogEntryV2::new_update(&Node::child(&self.path, field_number), value)
    }

    #[inline]
    pub fn entry_map_remove(&self, field_number: u32) -> LogEntryV2 {
        LogEntryV2::new_map_remove(&Node::child(&self.path, field_number))
    }

    impl_log!(log_update, entry_update, value: &impl SerializeV2);
    impl_log!(
        log_update_child,
        entry_update_child,
        field_number: u32,
        value: &impl SerializeV2,
    );
    impl_log!(log_list_push, entry_list_push, item: &impl SerializeV2);
    impl_log!(log_list_pop, entry_list_pop);
    impl_log!(log_map_remove, entry_map_remove, field_number: u32);
}

impl PartialEq for RuntimeV2 {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for RuntimeV2 {}

impl Default for RuntimeV2 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for RuntimeV2 {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl fmt::Debug for RuntimeV2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RuntimeV2")
            .field("logger", &"<logger>")
            .field("path", &*self.path)
            .finish()
    }
}
