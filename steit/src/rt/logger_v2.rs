use std::{
    io,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::log::{LogEntryV2, LoggerV2};

pub type LoggerHandleV2<T> = Arc<Mutex<RuntimeLoggerV2<T>>>;

pub trait PausableLoggerV2: LoggerV2 {
    fn pause(&mut self) -> u32;
    fn unpause(&mut self) -> u32;
}

pub struct RuntimeLoggerV2<T: LoggerV2> {
    logger: T,
    paused: u32,
}

impl<T: LoggerV2> RuntimeLoggerV2<T> {
    #[inline]
    pub(super) fn new(logger: T) -> Self {
        Self { logger, paused: 0 }
    }
}

impl<T: LoggerV2> LoggerV2 for RuntimeLoggerV2<T> {
    #[inline]
    fn log(&mut self, entry: LogEntryV2) -> io::Result<()> {
        if self.paused == 0 {
            self.logger.log(entry)?;
        }

        Ok(())
    }
}

impl<T: LoggerV2> PausableLoggerV2 for RuntimeLoggerV2<T> {
    #[inline]
    fn pause(&mut self) -> u32 {
        self.paused += 1;
        self.paused
    }

    #[inline]
    fn unpause(&mut self) -> u32 {
        if self.paused > 0 {
            self.paused -= 1;
        }

        self.paused
    }
}

impl<T: LoggerV2> Deref for RuntimeLoggerV2<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.logger
    }
}

impl<T: LoggerV2> DerefMut for RuntimeLoggerV2<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logger
    }
}
