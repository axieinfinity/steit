use crate::{
    log::{LogEntry, Logger},
    Serialize,
};

#[derive(Default)]
pub struct BufferLogger {
    log: Vec<LogEntry>,
}

impl BufferLogger {
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
    pub fn pluck(&mut self) -> Vec<u8> {
        let bytes = self.bytes();
        self.clear();
        bytes
    }
}

impl Logger for BufferLogger {
    #[inline]
    fn log(&mut self, entry: LogEntry) -> std::io::Result<()> {
        self.log.push(entry);
        Ok(())
    }
}
