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
            cached_size: CachedSize::unset(),
        }
    }
}

impl WireType for Child {
    const WIRE_TYPE: u8 = <u16 as WireType>::WIRE_TYPE;
}

impl Serialize for Child {
    #[inline]
    fn size(&self) -> u32 {
        self.tag.size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.tag.serialize(writer)
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
            cached_size: CachedSize::unset(),
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
    fn size(&self) -> u32 {
        0
    }

    #[inline]
    fn serialize(&self, _writer: &mut impl io::Write) -> io::Result<()> {
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

    #[inline]
    pub fn get_or_set_cached_size_from(&self, f: impl FnOnce() -> u32) -> u32 {
        match &*self.path {
            Node::Root { inner } => inner.value().cached_size.get_or_set_from(f),
            Node::Child { inner, .. } => inner.value().cached_size.get_or_set_from(f),
        }
    }

    #[inline]
    pub fn clear_cached_size(&self) {
        Self::clear_cached_size_branch(&self.path);
    }

    fn clear_cached_size_branch(path: &Node<Child, Root>) {
        match path {
            Node::Root { inner } => inner.value().cached_size.clear(),
            Node::Child { parent, inner } => {
                inner.value().cached_size.clear();
                Self::clear_cached_size_branch(parent);
            }
        }
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
    fn size(&self) -> u32 {
        self.path.size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.path.serialize(writer)
    }
}

#[cfg(test)]
mod test {
    use crate::{rt::node::Node, Deserialize, Eof, Serialize};

    use super::Runtime;

    #[test]
    fn serialization() {
        let runtime = Runtime::new().nested(10).nested(20);
        let mut bytes = Vec::new();

        runtime.serialize(&mut bytes).unwrap();

        let path = Vec::<u16>::deserialize(&mut Eof::new(&*bytes)).unwrap();

        assert_eq!(&*path, &[10, 20]);
    }

    #[test]
    fn clear_cached_size_branch() {
        // 2 level deep `Runtime`
        let runtime = Runtime::new().nested(2);

        // Set cached sizes of both `Runtime` nodes
        match &*runtime.path {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner, .. } => {
                inner.value().cached_size.set(7);

                match &**parent {
                    Node::Root { inner } => inner.value().cached_size.set(6),
                    Node::Child { .. } => assert!(false),
                }
            }
        }

        runtime.parent().clear_cached_size();

        match &*runtime.path {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner, .. } => {
                // Cached size of the leaf `Runtime` is still set.
                assert!(inner.value().cached_size.is_set());

                match &**parent {
                    // Cached size of the root `Runtime` has been cleared.
                    Node::Root { inner } => assert!(!inner.value().cached_size.is_set()),
                    Node::Child { .. } => assert!(false),
                }
            }
        };

        runtime.clear_cached_size();

        match &*runtime.path {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner, .. } => {
                // Now cached size of the leaf runtime has also been cleared.
                assert!(!inner.value().cached_size.is_set());

                match &**parent {
                    Node::Root { inner } => assert!(!inner.value().cached_size.is_set()),
                    Node::Child { .. } => assert!(false),
                }
            }
        };
    }
}
