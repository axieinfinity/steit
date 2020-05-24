use std::{
    io,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::log::{LogEntry, Logger};

pub type LoggerHandle<T> = Arc<Mutex<RuntimeLogger<T>>>;

pub trait PausableLogger: Logger {
    fn pause(&mut self) -> u32;
    fn unpause(&mut self) -> u32;
}

pub struct RuntimeLogger<T: Logger> {
    logger: T,
    paused: u32,
}

impl<T: Logger> RuntimeLogger<T> {
    pub(super) fn new(logger: T) -> Self {
        Self { logger, paused: 0 }
    }
}

impl<T: Logger> Logger for RuntimeLogger<T> {
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        if self.paused == 0 {
            self.logger.log(entry)?;
        }

        Ok(())
    }
}

impl<T: Logger> PausableLogger for RuntimeLogger<T> {
    fn pause(&mut self) -> u32 {
        self.paused += 1;
        self.paused
    }

    fn unpause(&mut self) -> u32 {
        if self.paused > 0 {
            self.paused -= 1;
        }

        self.paused
    }
}

impl<T: Logger> Deref for RuntimeLogger<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.logger
    }
}

impl<T: Logger> DerefMut for RuntimeLogger<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logger
    }
}
