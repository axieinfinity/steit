use std::{io, ops, rc::Rc};

use iowrap::Eof;

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Deserialize2, Serialize2,
};

use super::{cached_size::CachedSize, node::Node};

#[derive(Debug)]
struct Child {
    tag: u16,
    /// Cached size of the serialized object
    /// which the current `Runtime` object attaches to
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

impl Serialize2 for Child {
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
    /// which the current `Runtime` object attaches to
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

impl Serialize2 for Root {
    #[inline]
    fn size(&self) -> u32 {
        0
    }

    #[inline]
    fn serialize(&self, _writer: &mut impl io::Write) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Runtime {
    node: Rc<Node<Child, Root>>,
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn nested(&self, tag: u16) -> Self {
        Self {
            node: Rc::new(Node::child(&self.node, Child::new(tag))),
        }
    }

    #[inline]
    pub fn parent(&self) -> Self {
        Self {
            node: self.node.parent().expect("expect a parent `Runtime`"),
        }
    }

    #[inline]
    pub fn clear_cached_size(&self) {
        Self::clear_cached_size_branch(&self.node);
    }

    fn clear_cached_size_branch(node: &Node<Child, Root>) {
        match node {
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
    const WIRE_TYPE: u8 = <Node<Child, Root> as WireType>::WIRE_TYPE;
}

impl Serialize2 for Runtime {
    #[inline]
    fn size(&self) -> u32 {
        self.node.size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.node.serialize(writer)
    }
}

#[derive(Default)]
pub struct Path {
    path: Vec<u16>,
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl ops::Deref for Path {
    type Target = <Vec<u16> as ops::Deref>::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl WireType for Path {
    const WIRE_TYPE: u8 = <Runtime as WireType>::WIRE_TYPE;
}

impl Deserialize2 for Path {
    #[inline]
    fn with_runtime(_runtime: Runtime) -> Self {
        Self::new()
    }

    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let tag = u16::deserialize(reader)?;
            self.path.push(tag);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Deserialize2, Eof, Node, Path, Runtime, Serialize2};

    #[test]
    fn serialization() {
        let runtime = Runtime::new().nested(10).nested(20);
        let mut bytes = Vec::new();

        runtime.serialize(&mut bytes).unwrap();

        let path = Path::deserialize(&mut Eof::new(&*bytes)).unwrap();

        assert_eq!(&*path, &[10, 20]);
    }

    #[test]
    fn clear_cached_size_branch() {
        // 2 level deep `Runtime`
        let runtime = Runtime::new().nested(2);

        // Set cached sizes of both `Runtime` nodes
        match &*runtime.node {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner } => {
                inner.value().cached_size.set(7);

                match &**parent {
                    Node::Root { inner } => inner.value().cached_size.set(6),
                    Node::Child { .. } => assert!(false),
                }
            }
        }

        runtime.parent().clear_cached_size();

        match &*runtime.node {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner } => {
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

        match &*runtime.node {
            Node::Root { .. } => assert!(false),
            Node::Child { parent, inner } => {
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
