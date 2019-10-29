use std::rc::{Rc, Weak};

#[derive(Debug)]
enum Node<T> {
    Root,
    Child { parent: Weak<Self>, value: T },
}

impl<T> Node<T> {
    pub fn root() -> Self {
        Node::Root
    }

    pub fn child(parent: &Rc<Self>, value: T) -> Self {
        Node::Child {
            parent: Rc::downgrade(parent),
            value,
        }
    }

    fn push_values(&self, values: &mut Vec<T>)
    where
        T: Clone,
    {
        match self {
            Node::Root => {}
            Node::Child { parent, value } => {
                if let Some(parent) = parent.upgrade() {
                    parent.push_values(values);
                }

                values.push(value.clone());
            }
        }
    }

    pub fn to_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut values = Vec::new();
        self.push_values(&mut values);
        values
    }
}

impl<T: Clone + PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_values() == other.to_values()
    }
}

#[derive(Debug, PartialEq)]
pub struct Path {
    // We don't use borrow and lifetime here
    // since we don't want users to carry that burden over to their states.
    node: Rc<Node<u16>>,
}

impl Path {
    pub fn root() -> Self {
        Self {
            node: Rc::new(Node::root()),
        }
    }

    pub fn child(&self, tag: u16) -> Self {
        Self {
            node: Rc::new(Node::child(&self.node, tag)),
        }
    }
}
