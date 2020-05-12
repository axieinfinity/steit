use std::{fmt, sync::Arc};

pub enum Node<T> {
    Root,
    Child { parent: Arc<Self>, value: T },
}

impl<T> Node<T> {
    #[inline]
    pub fn child(parent: &Arc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
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

impl<T: Copy> Node<T> {
    fn write_values(&self, values: &mut Vec<T>) {
        match self {
            Node::Root => (),
            Node::Child { parent, value, .. } => {
                parent.write_values(values);
                values.push(*value);
            }
        }
    }

    #[inline]
    pub fn values(&self) -> Vec<T> {
        let mut values = Vec::new();
        self.write_values(&mut values);
        values
    }
}

struct DebugNode<'a, T>(&'a Node<T>);

impl<'a, T: fmt::Debug> fmt::Debug for DebugNode<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Node::Root => f.debug_struct("Root").finish(),
            Node::Child { value, .. } => f.debug_struct("Child").field("value", value).finish(),
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
