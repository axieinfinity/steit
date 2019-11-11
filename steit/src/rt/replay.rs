use std::io::{self, Read};

use iowrap::Eof;

use crate::{wire_type::WIRE_TYPE_SIZED, Merge, WireType};

pub enum ReplayKind {
    Update,
    Add,
    Remove,
}

#[derive(Default, Debug)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl WireType for Bytes {
    const WIRE_TYPE: u8 = WIRE_TYPE_SIZED;
}

impl Merge for Bytes {
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        reader.read_to_end(&mut self.bytes)?;
        Ok(())
    }
}

#[crate::steitize(Deserialize, own_crate, no_runtime)]
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
    pub fn decompose(self) -> (Vec<u16>, ReplayKind, Vec<u8>) {
        match self {
            ReplayEntry::Update { path, value } => (path, ReplayKind::Update, value.bytes),
            ReplayEntry::Add { path, item } => (path, ReplayKind::Add, item.bytes),
            ReplayEntry::Remove { path } => (path, ReplayKind::Remove, Vec::new()),
        }
    }
}
