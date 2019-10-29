use std::fmt;

use crate::{log::Logger, path::Path};

pub struct Runtime {
    logger: Logger,
    path: Path,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
            path: Path::root(),
        }
    }

    pub fn nested(&self, tag: u16) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.down(tag),
        }
    }
}

impl PartialEq for Runtime {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime {{ path: {:?} }}", self.path)
    }
}
