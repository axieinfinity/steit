use std::{fmt, io, ops, rc::Rc};

use iowrap::Eof;

use crate::{wire_type::WireType, CachedSize, Deserialize2, Serialize2};

enum Node<T> {
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

impl<T: Clone> Node<T> {
    #[inline]
    pub fn to_values(&self) -> Vec<T> {
        let mut values = Vec::new();
        self.push_values(&mut values);
        values
    }

    fn push_values(&self, values: &mut Vec<T>) {
        if let Node::Child { parent, value, .. } = self {
            parent.push_values(values);
            values.push(value.clone());
        }
    }
}

impl<T> Default for Node<T> {
    #[inline]
    fn default() -> Self {
        Node::Root
    }
}

impl<T> WireType for Node<T> {}

impl<T: Serialize2> Serialize2 for Node<T> {
    fn size(&self) -> u32 {
        match self {
            Node::Root => 0,
            Node::Child {
                parent,
                value,
                cached_size,
            } => cached_size.get_or_set_from(|| parent.size() + value.size()),
        }
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Node::Root => Ok(()),
            Node::Child { parent, value, .. } => {
                parent.serialize(writer)?;
                value.serialize(writer)
            }
        }
    }
}

#[derive(Default)]
pub struct Path {
    // We don't use borrow and lifetime here because:
    // 1. We don't want users to add more complexity from our side to their state objects.
    // 2. That would cause a circular reference from nested state objects to their parents.
    node: Rc<Node<u16>>,
}

impl Path {
    #[inline]
    pub fn root() -> Self {
        Default::default()
    }

    #[inline]
    pub fn child(&self, tag: u16) -> Self {
        Self {
            node: Rc::new(Node::child(&self.node, tag)),
        }
    }

    #[inline]
    pub fn parent(&self) -> Self {
        Self {
            node: self.node.parent().expect("expected a parent path"),
        }
    }

    #[inline]
    pub fn to_values(&self) -> Vec<u16> {
        self.node.to_values()
    }
}

impl PartialEq for Path {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Path {}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path {:?}", self.to_values())
    }
}

impl WireType for Path {
    const WIRE_TYPE: u8 = <Node<u16> as WireType>::WIRE_TYPE;
}

impl Serialize2 for Path {
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
pub struct RawPath {
    path: Vec<u16>,
}

impl RawPath {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl ops::Deref for RawPath {
    type Target = <Vec<u16> as ops::Deref>::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl WireType for RawPath {}

impl Deserialize2 for RawPath {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            let tag = u16::deserialize(reader)?;
            self.path.push(tag);
        }

        Ok(())
    }
}
