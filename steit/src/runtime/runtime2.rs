use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize2,
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

#[cfg(test)]
mod tests {
    use crate::Serialize2;

    use super::{Node, Runtime};

    #[test]
    fn test() {
        let runtime = Runtime::new();
        println!("{:?}", runtime);

        let runtime = runtime.nested(10);
        println!("{:?}", runtime);

        let mut bytes = Vec::new();

        runtime.serialize(&mut bytes).unwrap();
        println!("{:?}", runtime);
        println!("{:?}", bytes);

        println!("{}", runtime.size());
        println!("{:?}", runtime);

        match &*runtime.node {
            Node::Root { inner } => inner.value().cached_size.set(6),
            Node::Child { inner, .. } => inner.value().cached_size.set(7),
        }

        println!("{:?}", runtime);

        runtime.clear_cached_size();
        println!("{:?}", runtime);
    }
}
