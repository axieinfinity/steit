use std::{
    fmt, io,
    sync::{Arc, Mutex},
};

use crate::{
    log::{loggers::PrintLogger, LogEntry, Logger},
    wire_type::WireType,
    Serialize,
};

use super::{
    logger::{LoggerHandle, PausableLogger, RuntimeLogger},
    node::Node,
};

#[derive(Clone)]
pub struct Runtime {
    logger: Arc<Mutex<dyn PausableLogger>>,
    path: Arc<Node<u16>>,
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Self::with_logger_thrown(PrintLogger::with_stdout())
    }

    #[inline]
    pub fn with_logger<T: Logger + 'static>(logger: T) -> (Self, LoggerHandle<T>) {
        let logger = Arc::new(Mutex::new(RuntimeLogger::new(logger)));

        let runtime = Self {
            logger: logger.clone(),
            path: Arc::new(Node::Root),
        };

        (runtime, logger)
    }

    #[inline]
    pub fn with_logger_thrown<T: Logger + 'static>(logger: T) -> Self {
        Self::with_logger(logger).0
    }

    #[inline]
    pub fn nested(&self, tag: u16) -> Self {
        Self {
            logger: self.logger.clone(),
            path: Arc::new(Node::child(&self.path, tag)),
        }
    }

    #[inline]
    pub fn parent(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.parent().expect("expect a parent `Runtime`"),
        }
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
    pub fn log_update(&self, tag: u16, value: &impl Serialize) -> io::Result<()> {
        self.logger.lock().unwrap().log(LogEntry::new_update(
            Arc::new(Node::child(&self.path, tag)),
            value,
        ))
    }

    #[inline]
    pub fn log_update_in_place(&self, value: &impl Serialize) -> io::Result<()> {
        self.logger
            .lock()
            .unwrap()
            .log(LogEntry::new_update(self.path.clone(), value))
    }

    #[inline]
    pub fn log_add(&self, item: &impl Serialize) -> io::Result<()> {
        self.logger
            .lock()
            .unwrap()
            .log(LogEntry::new_add(self.path.clone(), item))
    }

    #[inline]
    pub fn log_remove(&self, tag: u16) -> io::Result<()> {
        self.logger
            .lock()
            .unwrap()
            .log(LogEntry::new_remove(Arc::new(Node::child(&self.path, tag))))
    }

    #[inline]
    pub fn pause_logger(&self) -> u32 {
        self.logger.lock().unwrap().pause()
    }

    #[inline]
    pub fn unpause_logger(&self) -> u32 {
        self.logger.lock().unwrap().unpause()
    }
}

impl PartialEq for Runtime {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Runtime {}

impl Default for Runtime {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Runtime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.path.compute_size();
        self.path.fmt(f)
    }
}

impl WireType for Runtime {
    const WIRE_TYPE: u8 = Node::<u16>::WIRE_TYPE;
}

impl Serialize for Runtime {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.path.compute_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.path.serialize_with_cached_size(writer)
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.path.cached_size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.path.serialize(writer)
    }
}
