use std::{fmt, sync::Arc};

pub enum Node<T> {
    Root,
    Child { parent: Arc<Self>, value: T },
}

impl<T> Node<T> {
    pub fn child(parent: &Arc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
        }
    }

    pub fn get_parent(&self) -> Option<Arc<Self>> {
        match self {
            Node::Root => None,
            Node::Child { parent, .. } => Some(parent.clone()),
        }
    }

    pub fn parent(&self) -> Arc<Self> {
        self.get_parent()
            .expect("there is no parent node of the root")
    }

    pub fn get_value(&self) -> Option<&T> {
        match self {
            Node::Root => None,
            Node::Child { value, .. } => Some(value),
        }
    }

    pub fn value(&self) -> &T {
        self.get_value().expect("root node doesn't have any value")
    }
}

impl<T: Copy> Node<T> {
    fn collect_values_to(&self, values: &mut Vec<T>) {
        match self {
            Node::Root => (),
            Node::Child { parent, value, .. } => {
                parent.collect_values_to(values);
                values.push(*value);
            }
        }
    }

    pub fn collect_values(&self) -> Vec<T> {
        let mut values = Vec::new();
        self.collect_values_to(&mut values);
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
