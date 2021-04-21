//! # Tacit
//! Obvious, powerful logging focused on structure and simplicity.

mod loggers;
mod outputs;

use log::{Log, Metadata, Record};
pub use loggers::*;
pub use outputs::*;

/// Describes all loggers provided by `tacit`
pub trait Logger: Log {
    /// Return the `LevelFilter` for the `Logger`
    fn level_filter(&self) -> log::LevelFilter;

    /// Set the `LevelFilter` for the `Logger`
    fn set_level_filter(&mut self, level: log::LevelFilter);

    /// Set the `LevelFilter` for the `Logger`, useful for chaining operations
    fn with_level_filter(mut self, level: log::LevelFilter) -> Self
    where
        Self: Sized,
    {
        self.set_level_filter(level);
        self
    }

    /// Set the `LevelFilter` for a particular module name.
    fn set_module_level_filter(&mut self, module: String, level: log::LevelFilter);

    /// Set the `LevelFilter` for a particular module name. Useful for chaining operations.
    fn with_module_level_filter(mut self, module: String, level: log::LevelFilter) -> Self
    where
        Self: Sized,
    {
        self.set_module_level_filter(module, level);
        self
    }

    /// Add a dynamic property to the logging output.
    fn add_fn_prop(&mut self, name: String, prop: fn(&Record) -> StaticProperty);

    /// Add a dynamic property to the logging output. Useful for chaining operations.
    fn with_fn_prop(mut self, name: String, prop: fn(&Record) -> StaticProperty) -> Self
    where
        Self: Sized,
    {
        self.add_fn_prop(name, prop);
        self
    }

    /// Add a static property to the logging output.
    fn add_prop(&mut self, name: String, prop: StaticProperty);

    /// Add a static property to the logging output. Useful for chaining operations.
    fn with_prop(mut self, name: String, prop: StaticProperty) -> Self
    where
        Self: Sized,
    {
        self.add_prop(name, prop);
        self
    }

    /// Only log from modules with an explicit module level filter, useful for quieting down
    /// dependencies.
    fn explicit_logging(&mut self);

    /// Only log from modules with an explicit module level filter, useful for quieting down
    /// dependencies.
    fn with_explicit_logging(mut self) -> Self
    where
        Self: Sized,
    {
        self.explicit_logging();
        self
    }

    /// Prepare the `Logger` for logging operations
    fn finalize(self) -> Self
    where
        Self: Sized;
}

/// Main logger abstraction for Tacit. Combines one or more `Logger` implementations.
pub struct TacitLogger {
    loggers: Vec<Box<dyn Logger>>,
    max_level: log::LevelFilter,
}

impl Default for TacitLogger {
    fn default() -> Self {
        Self {
            loggers: Vec::new(),
            max_level: log::LevelFilter::Off,
        }
    }
}

pub fn new() -> TacitLogger {
    TacitLogger::default()
}

impl TacitLogger {
    /// Add a logger to the pile
    pub fn with_logger<L: 'static + Logger>(mut self, logger: L) -> Self {
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

impl Log for TacitLogger {
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level().to_level_filter() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            for logger in &self.loggers {
                logger.log(record);
            }
        }
    }
}

/// Property to add to the log output
pub enum StaticProperty {
    String(String),
    Number(i64),
    Null,
}

impl From<String> for StaticProperty {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for StaticProperty {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Option<String>> for StaticProperty {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::String(s),
            None => Self::Null,
        }
    }
}

impl From<Option<&str>> for StaticProperty {
    fn from(s: Option<&str>) -> Self {
        match s {
            Some(s) => Self::String(s.to_string()),
            None => Self::Null,
        }
    }
}

impl From<i64> for StaticProperty {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}

impl From<u32> for StaticProperty {
    fn from(n: u32) -> Self {
        Self::Number(n as i64)
    }
}

pub enum Property {
    Static(StaticProperty),
    Function(Box<dyn Fn(&Record) -> StaticProperty + Send + Sync>),
}

#[cfg(test)]
mod tests {}
