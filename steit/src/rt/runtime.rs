use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize,
};

use super::{
    cached_size::CachedSize,
    log::{LogEntry, Logger},
    node::Node,
};

#[derive(Debug)]
struct Child {
    tag: u16,
    /// Cached size of the serialized object
    /// which the current `Runtime` attaches to
    cached_size: CachedSize,
}

impl Child {
    #[inline]
    pub fn new(tag: u16) -> Self {
        Self {
            tag,
            cached_size: CachedSize::new(),
        }
    }
}

impl WireType for Child {
    const WIRE_TYPE: u8 = <u16 as WireType>::WIRE_TYPE;
}

impl Serialize for Child {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.tag.compute_size()
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.tag.cached_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.tag.serialize_with_cached_size(writer)
    }
}

#[derive(Debug)]
struct Root {
    /// Cached size of the serialized object
    /// which the current `Runtime` attaches to
    cached_size: CachedSize,
}

impl Root {
    #[inline]
    pub fn new() -> Self {
        Self {
            cached_size: CachedSize::new(),
        }
    }
}

impl Default for Root {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl WireType for Root {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Serialize for Root {
    #[inline]
    fn compute_size(&self) -> u32 {
        0
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.compute_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, _writer: &mut impl io::Write) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct Runtime {
    logger: Logger,
    path: Rc<Node<Child, Root>>,
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn nested(&self, tag: u16) -> Self {
        Self {
            logger: self.logger.clone(),
            path: Rc::new(Node::child(&self.path, Child::new(tag))),
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
    pub fn set_cached_size(&self, size: u32) {
        match &*self.path {
            Node::Root { inner } => inner.value().cached_size.set(size),
            Node::Child { inner, .. } => inner.value().cached_size.set(size),
        }
    }

    #[inline]
    pub fn cached_size(&self) -> u32 {
        match &*self.path {
            Node::Root { inner } => inner.value().cached_size.get(),
            Node::Child { inner, .. } => inner.value().cached_size.get(),
        }
    }

    #[inline]
    pub fn log_update(&self, tag: u16, value: &impl Serialize) -> io::Result<()> {
        self.logger
            .log_entry(LogEntry::new_update(&self.nested(tag), value))
    }

    #[inline]
    pub fn log_update_in_place(&self, value: &impl Serialize) -> io::Result<()> {
        self.logger.log_entry(LogEntry::new_update(self, value))
    }

    #[inline]
    pub fn log_add(&self, item: &impl Serialize) -> io::Result<()> {
        self.logger.log_entry(LogEntry::new_add(self, item))
    }

    #[inline]
    pub fn log_remove(&self, tag: u16) -> io::Result<()> {
        self.logger
            .log_entry(LogEntry::new_remove(&self.nested(tag)))
    }
}

impl PartialEq for Runtime {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Runtime {}

impl WireType for Runtime {
    const WIRE_TYPE: u8 = <Node<u16> as WireType>::WIRE_TYPE;
}

impl Serialize for Runtime {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.path.compute_size()
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.path.cached_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.path.serialize_with_cached_size(writer)
    }
}

#[cfg(test)]
mod test {
    use crate::{Deserialize, Eof, Serialize};

    use super::Runtime;

    #[test]
    fn serialization() {
        let runtime = Runtime::new().nested(10).nested(20);
        let mut bytes = Vec::new();

        runtime.serialize(&mut bytes).unwrap();

        let path = Vec::<u16>::deserialize(&mut Eof::new(&*bytes)).unwrap();

        assert_eq!(&*path, &[10, 20]);
    }
}
