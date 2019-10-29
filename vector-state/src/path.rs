use std::{
    io,
    rc::{Rc, Weak},
};

use crate::{ser::Serialize, varint::Varint};
use std::io::Read;

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
    // since we don't want users to add more complexity from our side to their states.
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

    pub fn to_values(&self) -> Vec<u16> {
        self.node.to_values()
    }

    fn size(node: &Rc<Node<u16>>) -> u32 {
        match node.as_ref() {
            Node::Root => 0,
            Node::Child { parent, value } => {
                let mut size = Varint::size(value) as u32;

                if let Some(parent) = parent.upgrade() {
                    size += Self::size(&parent)
                }

                size
            }
        }
    }

    fn serialize<W: io::Write>(node: &Rc<Node<u16>>, writer: &mut W) -> io::Result<()> {
        match node.as_ref() {
            Node::Root => {}
            Node::Child { parent, value } => {
                if let Some(parent) = parent.upgrade() {
                    Self::serialize(&parent, writer)?;
                }

                Varint::serialize(value, writer)?;
            }
        }

        Ok(())
    }
}

impl Serialize for Path {
    fn size(&self) -> u32 {
        Self::size(&self.node)
    }

    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        Varint::serialize(&self.size(), writer)?;
        Self::serialize(&self.node, writer)
    }
}

pub fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Vec<u16>> {
    let size = Varint::deserialize(reader)?;
    let reader = &mut iowrap::Eof::new(reader.by_ref().take(size));
    let mut segments = Vec::new();

    while !reader.eof()? {
        let segment = Varint::deserialize(reader)?;
        segments.push(segment);
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use crate::ser::Serialize;

    use super::{deserialize, Path};

    macro_rules! t {
        ($name:ident : $segments:expr) => {
            #[test]
            fn $name() {
                assert_back_and_forth($segments);
            }
        };
    }

    fn assert_back_and_forth(segments: &[u16]) {
        let mut paths = vec![Path::root()];

        for segment in segments {
            let path = paths.last().unwrap().child(*segment);
            paths.push(path);
        }

        let path = paths.last().unwrap();
        let mut bytes = Vec::new();

        Serialize::serialize(path, &mut bytes).unwrap();

        assert_eq!(deserialize(&mut &*bytes).unwrap(), segments);
    }

    t!(back_and_forth_01: &[0]);
    t!(back_and_forth_02: &[1]);
    t!(back_and_forth_03: &[0, 0, 0, 1]);
    t!(back_and_forth_04: &[1, 1, 1]);
    t!(back_and_forth_05: &[10_000, -1i8 as u16, 137, 1, 2, 3, 4, 0, 0, 42]);
}
