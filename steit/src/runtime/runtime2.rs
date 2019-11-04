use std::{io, rc::Rc};

use crate::{runtime::node::Node, wire_type::WireType, Serialize2};

use super::cached_size::CachedSize;

#[derive(Debug)]
struct RuntimeInner {
    tag: u16,
    /// Cached size of the actual serialized object
    cached_size: CachedSize,
}

impl RuntimeInner {
    pub fn new(tag: u16) -> Self {
        Self {
            tag,
            cached_size: CachedSize::unset(),
        }
    }
}

impl WireType for RuntimeInner {
    const WIRE_TYPE: u8 = <u16 as WireType>::WIRE_TYPE;
}

impl Serialize2 for RuntimeInner {
    #[inline]
    fn size(&self) -> u32 {
        self.tag.size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.tag.serialize(writer)
    }
}

#[derive(Default, Debug)]
pub struct Runtime {
    node: Rc<Node<RuntimeInner>>,
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn nested(&self, tag: u16) -> Self {
        Self {
            node: Rc::new(Node::child(&self.node, RuntimeInner::new(tag))),
        }
    }

    pub fn parent(&self) -> Self {
        Self {
            node: self.node.parent().expect("expect a parent `Runtime`"),
        }
    }

    pub fn clear_cached_size(&self) {
        Self::clear_cached_size_branch(&self.node);
    }

    fn clear_cached_size_branch(node: &Node<RuntimeInner>) {
        match node {
            Node::Root => (),
            Node::Child { parent, value, .. } => {
                value.cached_size.clear();
                Self::clear_cached_size_branch(parent);
            }
        }
    }
}

impl WireType for Runtime {
    const WIRE_TYPE: u8 = <Node<RuntimeInner> as WireType>::WIRE_TYPE;
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

    use super::Runtime;

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

        runtime.clear_cached_size();
        println!("{:?}", runtime);
    }
}
