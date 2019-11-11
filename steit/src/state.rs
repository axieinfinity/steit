use std::io;

use iowrap::Eof;

use super::{
    de::Deserialize,
    rt::{
        replay::{ReplayEntry, ReplayKind},
        runtimed::Runtimed,
    },
    ser::Serialize,
    varint::Varint,
};

pub trait State: Runtimed + Serialize + Deserialize {
    fn replay_nested<'a>(
        &mut self,
        tag: u16,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()>;

    #[inline]
    fn replay(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        if !self.runtime().is_root() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "`replay` can only be called on the root `State` object",
            ));
        }

        while !reader.eof()? {
            let entry = ReplayEntry::deserialize(reader)?;
            println!("{:?}", entry);

            let (path, kind, buf) = entry.decompose();
            let path = &mut path.iter();
            let reader = &mut Eof::new(&*buf);

            if let Some(tag) = path.next() {
                self.replay_nested(*tag, path, &kind, reader)?;
            } else {
                match kind {
                    ReplayKind::Update => {
                        *self = Self::deserialize(reader)?;
                    }

                    ReplayKind::Add | ReplayKind::Remove => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "`add` and `remove` are not supported on the current `State` object",
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

impl<T: Varint> State for T {
    #[inline]
    fn replay_nested<'a>(
        &mut self,
        tag: u16,
        path: &mut impl Iterator<Item = &'a u16>,
        _kind: &ReplayKind,
        _reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()> {
        let mut s = format!("{}", tag);

        for tag in path {
            s.push_str(&format!(", {}", tag));
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected end-of-path but still got [{}] remaining", s),
        ))
    }
}
