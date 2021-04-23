//! # Logger
//! Combines a formatter and output to produce a log. Tacit can target multiple
//! loggers simultaneously.

use crate::{Property, StaticProperty, TacitFormatter, TacitOutput};
use log::{LevelFilter, Log, Metadata, Record};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

pub struct Logger<O: TacitOutput, F: TacitFormatter> {
    output: Arc<Mutex<O>>,
    formatter: F,
    msg_prop: String,
    default_props: Vec<(String, Property)>,
    max_level: LevelFilter,
    module_levels: HashMap<String, LevelFilter>,
    sorted_module_levels: Vec<(String, LevelFilter)>,
    explicit: bool,
}

impl<O: TacitOutput, F: TacitFormatter> Default for Logger<O, F> {
    fn default() -> Self {
        let mut logger = Self::new(O::default(), F::default());

        logger.add_fn_prop(String::from("timeStamp"), |_rec| {
            chrono::Utc::now().to_rfc3339().into()
        });

        logger.add_fn_prop(String::from("level"), |rec| rec.level().to_string().into());

        logger
    }
}

impl<O: TacitOutput, F: TacitFormatter> Logger<O, F> {
    pub fn new(output: O, formatter: F) -> Self {
        Self {
            output: Arc::new(Mutex::new(output)),
            msg_prop: String::from("msg"),
            formatter,
            default_props: Vec::new(),
            max_level: LevelFilter::Info,
            module_levels: HashMap::new(),
            sorted_module_levels: Vec::new(),
            explicit: false,
        }
    }
}

impl<O: TacitOutput, F: TacitFormatter> Log for Logger<O, F> {
    fn log(&self, record: &Record) {
        let mut output = self.output.lock();
        self.formatter
            .log(&mut *output, record, &self.msg_prop, &self.default_props);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        let module = metadata.target();

        let level = self
            .sorted_module_levels
            .iter()
            .find(|(item, _level)| module.starts_with(item))
            .map(|(_item, level)| level);

        let level = if self.explicit {
            level.unwrap_or(&LevelFilter::Off)
        } else {
            level.unwrap_or(&self.max_level)
        };

        level != &LevelFilter::Off && level >= &self.max_level
    }

    fn flush(&self) {}
}

/// Describes all loggers provided by `tacit`
impl<O: TacitOutput, F: TacitFormatter> Logger<O, F> {
    /// Return the `LevelFilter` for the `Logger`
    pub fn level_filter(&self) -> log::LevelFilter {
        if self.explicit {
            log::LevelFilter::Trace
        } else {
            self.max_level
        }
    }

    /// Set the `LevelFilter` for the `Logger`, useful for chaining operations
    pub fn with_level_filter(mut self, level: log::LevelFilter) -> Self
    where
        Self: Sized,
    {
        self.set_level_filter(level);
        self
    }

    pub fn set_level_filter(&mut self, level: log::LevelFilter) {
        self.max_level = level;
    }

    /// Set the `LevelFilter` for a particular module name.
    pub fn set_module_level_filter(&mut self, module: String, level: log::LevelFilter) {
        self.module_levels.insert(module, level);
    }

    /// Set the `LevelFilter` for a particular module name. Useful for chaining operations.
    pub fn with_module_level_filter(mut self, module: String, level: log::LevelFilter) -> Self
    where
        Self: Sized,
    {
        self.set_module_level_filter(module, level);
        self
    }

    /// Add a dynamic property to the logging output. Useful for chaining operations.
    pub fn with_fn_prop(mut self, name: String, prop: fn(&Record) -> StaticProperty) -> Self
    where
        Self: Sized,
    {
        self.add_fn_prop(name, prop);
        self
    }

    /// Add a dynamic property to the logging output.
    pub fn add_fn_prop(&mut self, name: String, prop: fn(&Record) -> StaticProperty) {
        self.default_props
            .push((name, Property::Function(Box::new(prop))));
    }

    /// Add a static property to the logging output.
    pub fn add_prop(&mut self, name: String, prop: StaticProperty) {
        self.default_props.push((name, Property::Static(prop)));
    }

    /// Add a static property to the logging output. Useful for chaining operations.
    pub fn with_prop(mut self, name: String, prop: StaticProperty) -> Self
    where
        Self: Sized,
    {
        self.add_prop(name, prop);
        self
    }

    /// Only log from modules with an explicit module level filter, useful for quieting down
    /// dependencies.
    pub fn explicit_logging(&mut self) {
        self.explicit = true;
    }

    /// Only log from modules with an explicit module level filter, useful for quieting down
    /// dependencies.
    pub fn with_explicit_logging(mut self) -> Self
    where
        Self: Sized,
    {
        self.explicit_logging();
        self
    }

    /// Prepare the `Logger` for logging operations
    pub(crate) fn finalize(mut self) -> Self {
        self.sorted_module_levels = self
            .module_levels
            .clone()
            .into_iter()
            .collect::<Vec<(String, LevelFilter)>>();

        self.module_levels.clear();

        self.sorted_module_levels
            .sort_by_key(|(name, _level)| name.len().wrapping_neg());

        self
    }
}
