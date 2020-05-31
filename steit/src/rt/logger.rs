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
    inner: T,
    paused: u32,
}

impl<T: Logger> RuntimeLogger<T> {
    pub(super) fn new(inner: T) -> Self {
        Self { inner, paused: 0 }
    }

    pub fn replace(&mut self, inner: T) {
        self.inner = inner;
    }
}

impl<T: Logger> Logger for RuntimeLogger<T> {
    fn log(&mut self, entry: LogEntry) -> io::Result<()> {
        if self.paused == 0 {
            self.inner.log(entry)?;
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
        &self.inner
    }
}

impl<T: Logger> DerefMut for RuntimeLogger<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
