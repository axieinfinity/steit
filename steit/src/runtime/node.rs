use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize,
};

use super::cached_size::CachedSize;

#[derive(Debug)]
pub struct NodeInner<T> {
    value: T,

    /// Cached size of the current branch, starting from this node's value to root.
    ///
    /// Since this tree structure is immutable, this cached size should never be touched.
    cached_size: CachedSize,
}

impl<T> NodeInner<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            cached_size: CachedSize::unset(),
        }
    }

    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub enum Node<Child, Root = Child> {
    Root {
        inner: NodeInner<Root>,
    },
    Child {
        parent: Rc<Self>,
        inner: NodeInner<Child>,
    },
}

impl<Child, Root> Node<Child, Root> {
    #[inline]
    pub fn root(value: Root) -> Self {
        Node::Root {
            inner: NodeInner::new(value),
        }
    }

    #[inline]
    pub fn child(parent: &Rc<Self>, value: Child) -> Self {
        Node::Child {
            parent: parent.clone(),
            inner: NodeInner::new(value),
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<Rc<Self>> {
        match self {
            Node::Root { .. } => None,
            Node::Child { parent, .. } => Some(parent.clone()),
        }
    }
}

impl<Child, Root: Default> Default for Node<Child, Root> {
    #[inline]
    fn default() -> Self {
        Self::root(Default::default())
    }
}

impl<Child, Root> WireType for Node<Child, Root> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<Child: Serialize, Root: Serialize> Serialize for Node<Child, Root> {
    fn size(&self) -> u32 {
        match self {
            Node::Root { inner } => inner
                .cached_size
                .get_or_set_from(|| inner.value.size_nested(None)),

            Node::Child { parent, inner } => inner
                .cached_size
                .get_or_set_from(|| parent.size() + inner.value.size_nested(None)),
        }
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root { inner } => inner.value.serialize_nested(None, writer),

            Node::Child { parent, inner, .. } => {
                parent.serialize(writer)?;
                inner.value.serialize_nested(None, writer)
            }
        }
    }
}
