use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize2,
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

impl<Child: Serialize2, Root: Serialize2> Serialize2 for Node<Child, Root> {
    fn size(&self) -> u32 {
        match self {
            Node::Root { inner } => inner.cached_size.get_or_set_from(|| {
                let mut size = 0;

                // TODO: Refactor this into a common utility
                if Root::WIRE_TYPE == WIRE_TYPE_SIZED {
                    if inner.value.size() > 0 {
                        size += inner.value.size().size();
                        size += inner.value.size();
                    }
                } else {
                    // TODO: Check for default varint value
                    size += inner.value.size();
                }

                size
            }),

            Node::Child { parent, inner } => inner.cached_size.get_or_set_from(|| {
                let mut size = parent.size();

                if Child::WIRE_TYPE == WIRE_TYPE_SIZED {
                    if inner.value.size() > 0 {
                        size += inner.value.size().size();
                        size += inner.value.size();
                    }
                } else {
                    size += inner.value.size();
                }

                size
            }),
        }
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root { inner } => {
                if Root::WIRE_TYPE == WIRE_TYPE_SIZED {
                    if inner.value.size() > 0 {
                        inner.value.size().serialize(writer)?;
                        inner.value.serialize(writer)?;
                    }
                } else {
                    inner.value.serialize(writer)?;
                }
            }

            Node::Child { parent, inner, .. } => {
                parent.serialize(writer)?;

                if Child::WIRE_TYPE == WIRE_TYPE_SIZED {
                    if inner.value.size() > 0 {
                        inner.value.size().serialize(writer)?;
                        inner.value.serialize(writer)?;
                    }
                } else {
                    inner.value.serialize(writer)?;
                }
            }
        }

        Ok(())
    }
}
