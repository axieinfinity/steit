use std::io;

use super::{
    de::Deserialize,
    rt::Runtime,
    ser::Serialize,
    types::{Bytes, Varint},
    Eof,
};

#[derive(PartialEq, Eq)]
pub enum ReplayKind {
    Update,
    Add,
    Remove,
}

#[crate::steitize(Deserialize, own_crate)]
#[derive(Debug)]
pub enum ReplayEntry {
    #[steit(tag = 0)]
    Update {
        #[steit(tag = 0)]
        path: Vec<u16>,
        #[steit(tag = 1)]
        value: Bytes,
    },
    #[steit(tag = 1)]
    Add {
        #[steit(tag = 0)]
        path: Vec<u16>,
        #[steit(tag = 1)]
        item: Bytes,
    },
    #[steit(tag = 2)]
    Remove {
        #[steit(tag = 0)]
        path: Vec<u16>,
    },
}

impl ReplayEntry {
    #[inline]
    pub fn decompose(self) -> (Vec<u16>, ReplayKind, Vec<u8>) {
        match self {
            ReplayEntry::Update { path, value, .. } => (path, ReplayKind::Update, value.bytes()),
            ReplayEntry::Add { path, item, .. } => (path, ReplayKind::Add, item.bytes()),
            ReplayEntry::Remove { path, .. } => (path, ReplayKind::Remove, Vec::new()),
        }
    }
}

pub trait State: Serialize + Deserialize {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;

    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()>;

    #[inline]
    fn is_root(&self) -> bool {
        self.runtime().is_root()
    }

    #[inline]
    fn is_child(&self) -> bool {
        !self.is_root()
    }

    #[inline]
    fn handle_update(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Self::with_runtime(self.runtime().clone());
        self.merge(reader)
    }

    #[inline]
    fn replay(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if !self.is_root() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = ReplayEntry::deserialize_nested(reader)?;
            println!("{:?}", entry);

            let (path, kind, buf) = entry.decompose();
            let path = &mut path.iter();
            let reader = &mut Eof::new(&*buf);

            self.handle(path, &kind, reader)?;
        }

        Ok(())
    }
}

impl<T: Varint> State for T {
    #[inline]
    fn with_runtime(_runtime: Runtime) -> Self {
        Self::default()
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        panic!("cannot get a `Runtime` from a varint")
    }

    #[inline]
    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()> {
        if let Some(tag) = path.next() {
            let mut s = format!("{}", tag);

            for tag in path {
                s.push_str(&format!(", {}", tag));
            }

            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected end-of-path but still got [{}] remaining", s),
            ))
        } else {
            match kind {
                ReplayKind::Update => self.handle_update(reader),

                ReplayKind::Add | ReplayKind::Remove => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "`add` and `remove` are not supported on varints",
                )),
            }
        }
    }

    #[inline]
    fn handle_update(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        *self = Self::deserialize(reader)?;
        Ok(())
    }
}
