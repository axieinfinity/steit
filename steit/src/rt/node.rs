use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    CachedSize, Serialize,
};

#[derive(Debug)]
pub enum Node<T> {
    Root,
    Child {
        parent: Rc<Self>,
        value: T,
        cached_size: CachedSize,
    },
}

impl<T> Node<T> {
    #[inline]
    pub fn child(parent: &Rc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<Rc<Self>> {
        match self {
            Node::Root => None,
            Node::Child { parent, .. } => Some(parent.clone()),
        }
    }
}

impl<T> Default for Node<T> {
    #[inline]
    fn default() -> Self {
        Node::Root
    }
}

impl<T> WireType for Node<T> {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl<T: Serialize> Serialize for Node<T> {
    fn compute_size(&self) -> u32 {
        match self {
            Node::Root => 0,
            Node::Child {
                parent,
                value,
                cached_size,
            } => {
                let size = parent.compute_size() + value.compute_size_nested(None);
                cached_size.set(size);
                size
            }
        }
    }

    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root => Ok(()),
            Node::Child { parent, value, .. } => {
                parent.serialize_with_cached_size(writer)?;
                value.serialize_nested_with_cached_size(None, writer)
            }
        }
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        match self {
            Node::Root => 0,
            Node::Child { cached_size, .. } => cached_size.get(),
        }
    }
}
