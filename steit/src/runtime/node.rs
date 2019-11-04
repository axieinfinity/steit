use std::{io, rc::Rc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    Serialize2,
};

use super::cached_size::CachedSize;

#[derive(Debug)]
pub enum Node<T> {
    Root,
    Child {
        parent: Rc<Self>,
        value: T,
        /// Cached size of the node itself
        cached_size: CachedSize,
    },
}

impl<T> Node<T> {
    #[inline]
    pub fn child(parent: &Rc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
            cached_size: CachedSize::unset(),
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

impl<T: Serialize2> Serialize2 for Node<T> {
    fn size(&self) -> u32 {
        match self {
            Node::Root => 0,
            Node::Child {
                parent,
                value,
                cached_size,
            } => cached_size.get_or_set_from(|| {
                let mut size = parent.size();

                if T::WIRE_TYPE == WIRE_TYPE_SIZED {
                    size += value.size().size();
                }

                size += value.size();
                size
            }),
        }
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root => Ok(()),
            Node::Child { parent, value, .. } => {
                parent.serialize(writer)?;

                if T::WIRE_TYPE == WIRE_TYPE_SIZED {
                    value.size().serialize(writer)?;
                }

                value.serialize(writer)
            }
        }
    }
}
