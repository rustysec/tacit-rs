//! # Tacit
//! Obvious, powerful logging focused on structure and simplicity.

mod formatters;
mod logger;
mod outputs;
mod properties;

pub use crate::{formatters::*, logger::*, outputs::*, properties::*};
use log::{Log, Metadata, Record};

/// Main logger abstraction for Tacit. Combines one or more `Logger` implementations.
pub struct TacitLogger<O: TacitOutput, F: TacitFormatter> {
    loggers: Vec<Box<Logger<O, F>>>,
    max_level: log::LevelFilter,
}

impl<O: TacitOutput, F: TacitFormatter> Default for TacitLogger<O, F> {
    fn default() -> Self {
        Self {
            loggers: Vec::new(),
            max_level: log::LevelFilter::Off,
        }
    }
}

pub fn new<O: TacitOutput, F: TacitFormatter>() -> TacitLogger<O, F> {
    TacitLogger::default()
}

impl<O: 'static + TacitOutput, F: 'static + TacitFormatter> TacitLogger<O, F> {
    /// Add a logger to the pile
    pub fn with_logger(mut self, logger: Logger<O, F>) -> Self {
        self.max_level = std::cmp::max(self.max_level, logger.level_filter());
        self.loggers.push(Box::new(logger.finalize()));
        self
    }

    /// Starts logging system so that  `log` macros work
    pub fn log(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(self.max_level);
        log::set_boxed_logger(Box::new(self))
    }
}

impl<O: TacitOutput, F: TacitFormatter> Log for TacitLogger<O, F> {
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level().to_level_filter() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            for logger in &self.loggers {
                if logger.enabled(record.metadata()) {
                    logger.log(record);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
