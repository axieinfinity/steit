use std::{cell::RefCell, fmt, io, rc::Rc};

use crate::{
    log::{loggers::PrintLogger, LogEntry, Logger},
    wire_type::WireType,
    Serialize,
};

use super::node::Node;

#[derive(Clone)]
pub struct Runtime {
    logger: Rc<RefCell<dyn Logger>>,
    path: Rc<Node<u16>>,
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Self {
            logger: Rc::new(RefCell::new(PrintLogger::with_stdout())),
            path: Rc::new(Node::Root),
        }
    }

    #[inline]
    pub fn nested(&self, tag: u16) -> Self {
        Self {
            logger: self.logger.clone(),
            path: Rc::new(Node::child(&self.path, tag)),
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
        self.logger
            .borrow_mut()
            .log(LogEntry::new_update(&self.nested(tag), value))
    }

    #[inline]
    pub fn log_update_in_place(&self, value: &impl Serialize) -> io::Result<()> {
        self.logger
            .borrow_mut()
            .log(LogEntry::new_update(self, value))
    }

    #[inline]
    pub fn log_add(&self, item: &impl Serialize) -> io::Result<()> {
        self.logger.borrow_mut().log(LogEntry::new_add(self, item))
    }

    #[inline]
    pub fn log_remove(&self, tag: u16) -> io::Result<()> {
        self.logger
            .borrow_mut()
            .log(LogEntry::new_remove(&self.nested(tag)))
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
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Runtime {
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
