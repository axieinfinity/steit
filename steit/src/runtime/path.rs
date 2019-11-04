use std::{fmt, io, rc::Rc};

use iowrap::Eof;

use crate::{varint, Serialize};

enum Node<T> {
    Root,
    Child { parent: Rc<Self>, value: T },
}

impl<T> Node<T> {
    pub fn child(parent: &Rc<Self>, value: T) -> Self {
        Node::Child {
            parent: parent.clone(),
            value,
        }
    }

    pub fn parent(&self) -> Option<Rc<Self>> {
        if let Node::Child { parent, .. } = self {
            Some(parent.clone())
        } else {
            None
        }
    }

    fn push_values(&self, values: &mut Vec<T>)
    where
        T: Clone,
    {
        match self {
            Node::Root => {}
            Node::Child { parent, value } => {
                parent.push_values(values);
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

pub struct Path {
    // We don't use borrow and lifetime here bacause:
    // 1. We don't want users to add more complexity from our side to their state objects.
    // 2. That would cause a circular reference from nested state objects to their parents.
    node: Rc<Node<u16>>,
}

impl Path {
    pub fn root() -> Self {
        Self {
            node: Rc::new(Node::Root),
        }
    }

    pub fn child(&self, tag: u16) -> Self {
        Self {
            node: Rc::new(Node::child(&self.node, tag)),
        }
    }

    pub fn parent(&self) -> Self {
        let node = self.node.parent().unwrap_or(Rc::new(Node::Root));
        Self { node }
    }

    pub fn to_values(&self) -> Vec<u16> {
        self.node.to_values()
    }

    fn size(node: &Node<u16>) -> u32 {
        match node {
            Node::Root => 0,
            Node::Child { parent, value } => value.size() + Self::size(&parent),
        }
    }

    fn serialize(node: &Node<u16>, writer: &mut impl io::Write) -> io::Result<()> {
        match node {
            Node::Root => Ok(()),
            Node::Child { parent, value } => {
                Self::serialize(&parent, writer)?;
                value.serialize(writer)
            }
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.to_values() == other.to_values()
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path {:?}", self.to_values())
    }
}

impl Serialize for Path {
    fn size(&self) -> u32 {
        Self::size(&self.node)
    }

    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        Self::serialize(&self.node, writer)
    }
}

pub fn deserialize(reader: &mut Eof<impl io::Read>) -> io::Result<Vec<u16>> {
    let mut path = Vec::new();

    while !reader.eof()? {
        let tag = varint::deserialize(reader)?;
        path.push(tag);
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use crate::{test_case, Eof, Serialize};

    use super::{deserialize, Path};

    fn assert_back_and_forth(tags: &[u16]) {
        let mut path = Path::root();

        for tag in tags {
            path = path.child(*tag);
        }

        let mut bytes = Vec::new();

        path.serialize(&mut bytes).unwrap();

        assert_eq!(deserialize(&mut Eof::new(&*bytes)).unwrap(), tags);
    }

    test_case!(back_and_forth_01: assert_back_and_forth; &[0]);
    test_case!(back_and_forth_02: assert_back_and_forth; &[1]);
    test_case!(back_and_forth_03: assert_back_and_forth; &[0, 0, 0, 1]);
    test_case!(back_and_forth_04: assert_back_and_forth; &[1, 1, 1]);
    test_case!(back_and_forth_05: assert_back_and_forth; &[10_000, -1i8 as u16, 137, 1, 2, 3, 4, 0, 0, 42]);
}
