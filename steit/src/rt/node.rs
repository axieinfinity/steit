use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize,
};

use super::cached_size::CachedSize;

#[derive(Debug)]
pub struct Inner<T> {
    value: T,

    /// Cached size of the current branch, starting from this node's value to root.
    ///
    /// Since this tree structure is immutable, this cached size should never be touched.
    cached_size: CachedSize,
}

impl<T> Inner<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            cached_size: CachedSize::new(),
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
        inner: Inner<Root>,
    },
    Child {
        parent: Rc<Self>,
        inner: Inner<Child>,
    },
}

impl<Child, Root> Node<Child, Root> {
    #[inline]
    pub fn root(value: Root) -> Self {
        Node::Root {
            inner: Inner::new(value),
        }
    }

    #[inline]
    pub fn child(parent: &Rc<Self>, value: Child) -> Self {
        Node::Child {
            parent: parent.clone(),
            inner: Inner::new(value),
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
        Self::root(Root::default())
    }
}

impl<Child, Root> WireType for Node<Child, Root> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<Child: Serialize, Root: Serialize> Serialize for Node<Child, Root> {
    fn compute_size(&self) -> u32 {
        match self {
            Node::Root { inner } => {
                let size = inner.value.compute_size_nested(None);
                inner.cached_size.set(size);
                size
            }

            Node::Child { parent, inner } => {
                let size = parent.compute_size() + inner.value.compute_size_nested(None);
                inner.cached_size.set(size);
                size
            }
        }
    }

    fn cached_size(&self) -> u32 {
        match self {
            Node::Root { inner } => inner.cached_size.get(),
            Node::Child { inner, .. } => inner.cached_size.get(),
        }
    }

    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root { inner } => inner.value.serialize_nested_with_cached_size(None, writer),

            Node::Child { parent, inner } => {
                parent.serialize_with_cached_size(writer)?;
                inner.value.serialize_nested_with_cached_size(None, writer)
            }
        }
    }
}
