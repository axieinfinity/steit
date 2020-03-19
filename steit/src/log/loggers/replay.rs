use crate::{
    log::{LogEntry, Logger},
    ReplayEntry, Serialize,
};

#[derive(Default)]
pub struct ReplayLogger {
    log: Vec<ReplayEntry>,
}

impl ReplayLogger {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn log_raw(&mut self, entry: ReplayEntry) {
        self.log.push(entry);
    }

    #[inline]
    pub fn log_raws(&mut self, mut entries: Vec<ReplayEntry>) {
        self.log.append(&mut entries);
    }

    #[inline]
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for entry in &self.log {
            entry.compute_size();
            entry
                .serialize_nested_with_cached_size(None, &mut bytes)
                .unwrap();
        }

        bytes
    }

    #[inline]
    pub fn clear(&mut self) {
        self.log.clear();
    }

    #[inline]
    pub fn pluck(&mut self) -> Vec<ReplayEntry> {
        std::mem::replace(&mut self.log, Vec::new())
    }
}

impl Logger for ReplayLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> std::io::Result<()> {
        self.log.push(entry.into());
        Ok(())
    }
}
