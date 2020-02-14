use std::{fmt, io, sync::Arc};

use crate::{
    wire_type::{WireType, WIRE_TYPE_SIZED},
    CachedSize, Serialize,
};

pub enum Node<T> {
    Root,
    Child {
        parent: Arc<Self>,
        value: T,
        cached_size: CachedSize,
    },
}

impl<T> Node<T> {
    #[inline]
    pub fn child(parent: &Arc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
            cached_size: CachedSize::new(),
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<Arc<Self>> {
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

struct DebugNode<'a, T>(&'a Node<T>);

impl<'a, T: fmt::Debug> fmt::Debug for DebugNode<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Node::Root => f.debug_struct("Root").finish(),
            Node::Child {
                value, cached_size, ..
            } => f
                .debug_struct("Child")
                .field("value", value)
                .field("cached_size", cached_size)
                .finish(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nodes = Vec::new();
        let mut node = self;

        while let Node::Child { parent, .. } = node {
            nodes.push(DebugNode(node));
            node = parent;
        }

        nodes.push(DebugNode(&Node::Root));
        nodes.reverse();

        f.debug_list().entries(nodes).finish()
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        test_case,
        test_utils::{assert_serialize, assert_size},
        Serialize,
    };

    use super::Node;

    fn node<T>(branch: impl IntoIterator<Item = T>) -> Node<T> {
        branch.into_iter().fold(Node::Root, |node, value| {
            Node::child(&Arc::new(node), value)
        })
    }

    #[test]
    fn cached_size() {
        let node = node(vec![0, 1337, 0, 1]);
        assert_eq!(node.cached_size(), 0);
        assert_eq!(node.compute_size(), 5);
        assert_eq!(node.cached_size(), 5);
    }

    test_case!(size_01: assert_size::<Node<i32>>; node(vec![]) => 0);
    test_case!(size_02: assert_size; node(vec![9]) => 1);
    test_case!(size_03: assert_size; node(vec![1337]) => 2);
    test_case!(size_04: assert_size; node(vec![9, 1337, 128]) => 5);

    test_case!(serialize_01: assert_serialize::<Node<i32>>; node(vec![]) => &[]);
    test_case!(serialize_02: assert_serialize; node(vec![9]) => &[18]);
    test_case!(serialize_03: assert_serialize; node(vec![1337]) => &[242, 20]);
    test_case!(serialize_04: assert_serialize; node(vec![9, 1337, 128]) => &[18, 242, 20, 128, 2]);
}
