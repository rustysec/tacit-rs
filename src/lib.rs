//! # Tacit
//!
//! An obvious, powerful logging library for Rust's [log](https://crates.io/crates/log) ecosystem.
//! Focused on structure and simplicity.
//!
//! ## Overview
//! There are a lot of great and very useful logging libraries for Rust.
//! However, some are a little too simple, others a little too complex.
//! `tacit` aims to walk the fine line with enough features to provide
//! the power most users would want, but with a simple and obvious
//! interface that makes discovering "how to use this" easy.
//!
//!
//! ## Principals
//! There are two main components that make up the `tacit` logging system.
//! These are [formatters](#formatters) and [outputs](#outputs).
//!
//!
//! ### Formatters
//! Loggers control the output format of the entries. This can be a simple
//! line of text, or something more structured like JSON or CEF.
//!
//!
//! ### Outputs
//! Outputs dictate where the log entries arrive. Examples include the console,
//! a file, an archive, a database, etc.
//!
//!
//! ## Usage
//! `tacit` has a very simple API surface, meant to provide enough options to be
//! usefully without overwhelming developers.
//!
//! [Formatters](#formatters) and [Outputs](#outputs) both implement `Default` so
//! you can be reasonably assured that logging will work with sane defaults. A
//! simple setup involves something like this:
//!
//! ```rust
//! use tacit::{JsonFormatter, Logger, SimpleConsoleOutput};
//!
//! let json_logger = Logger::<SimpleConsoleOutput, JsonFormatter>::default();
//! tacit::new().with_logger(json_logger).log().unwrap();
//! log::info!("logging some info!");
//! ```
//!
//! In the event that a [formatter](#formatters) or [output](#outputs) has specific
//! configuration options, they can be used like this:
//!
//! ```rust
//! use tacit::{JsonFormatter, Logger, SimpleConsoleOutput};
//!
//! let output = SimpleConsoleOutput::default(); // with options...
//! let formatter = JsonFormatter::default(); // with options...
//! let json_logger = Logger::new(output, formatter);
//! tacit::new().with_logger(json_logger).log().unwrap();
//! log::info!("logging some info!");
//! ```
//!

mod formatters;
mod logger;
mod outputs;
mod properties;

pub use crate::{formatters::*, logger::*, outputs::*, properties::*};
pub use log::LevelFilter;
use log::{Log, Metadata, Record};

#[cfg(feature = "kv")]
pub use kv_log_macro::{debug, error, info, trace, warn};

#[cfg(not(feature = "kv"))]
pub use log::{debug, error, info, trace, warn};

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
    #[must_use]
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
