use std::io;

use crate::{varint::Varint, Deserialize, RawEntryKind, Serialize};

pub trait State: Serialize + Deserialize {
    fn process_log<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &RawEntryKind,
        reader: &mut impl io::Read,
    ) -> io::Result<()>;
}

impl<T: Varint> State for T {
    fn process_log<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: &RawEntryKind,
        reader: &mut impl io::Read,
    ) -> io::Result<()> {
        if let Some(tag) = path.next() {
            let mut s = format!("{}", tag);

            for tag in path {
                s.push_str(&format!(", {}", tag));
            }

            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected end-of-path but still got [{}] remaining", s),
            ));
        }

        match kind {
            RawEntryKind::Update => {
                *self = Varint::deserialize(reader)?;
                Ok(())
            }

            RawEntryKind::Add => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("`add` is not supported for varints"),
            )),

            RawEntryKind::Remove => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("`remove` is not supported for varints"),
            )),
        }
    }
}
