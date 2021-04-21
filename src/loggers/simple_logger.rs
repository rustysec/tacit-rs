//! # Simple Logger
//! Structures log output in a simple parsable format.
//!
//! ```
//! timeStamp="2021-03-29T15:16:25.683809406+00:00" level="INFO" msg="logging a thing"
//! ```

use crate::{Logger, Property, SimpleConsoleOutput, StaticProperty, TacitOutput};
use log::{LevelFilter, Metadata, Record};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

pub struct SimpleLogger<O: TacitOutput> {
    output: Arc<Mutex<O>>,
    msg_prop: String,
    default_props: Vec<(String, Property)>,
    max_level: LevelFilter,
    module_levels: HashMap<String, LevelFilter>,
    sorted_module_levels: Vec<(String, LevelFilter)>,
    explicit: bool,
}

impl Default for SimpleLogger<SimpleConsoleOutput> {
    /// Creates the default Json logger with a `timeStamp`, `level`, and `msg` properties
    fn default() -> Self {
        let mut logger = Self::new();

        logger.add_fn_prop(String::from("timeStamp"), |_rec| {
            chrono::Utc::now().to_rfc3339().into()
        });

        logger.add_fn_prop(String::from("level"), |rec| rec.level().to_string().into());

        logger
    }
}

impl<O: TacitOutput> SimpleLogger<O> {
    /// Creates a new empty Json logger with no default properties other than `msg`
    pub fn new() -> Self {
        Self {
            output: Arc::new(Mutex::new(O::default())),
            msg_prop: String::from("msg"),
            default_props: Vec::new(),
            max_level: LevelFilter::Info,
            module_levels: HashMap::new(),
            sorted_module_levels: Vec::new(),
            explicit: false,
        }
    }
}

impl<O: TacitOutput> crate::Logger for SimpleLogger<O> {
    fn level_filter(&self) -> log::LevelFilter {
        self.max_level
    }

    fn set_level_filter(&mut self, level: log::LevelFilter) {
        self.max_level = level;
    }

    fn set_module_level_filter(&mut self, module: String, level: log::LevelFilter) {
        self.module_levels.insert(module, level);
    }

    fn add_fn_prop(&mut self, name: String, prop: fn(&Record) -> StaticProperty) {
        self.default_props
            .push((name, Property::Function(Box::new(prop))));
    }

    fn add_prop(&mut self, name: String, prop: StaticProperty) {
        self.default_props.push((name, Property::Static(prop)));
    }

    fn explicit_logging(&mut self) {
        self.explicit = true;
    }

    fn finalize(mut self) -> Self {
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

impl<O: TacitOutput> log::Log for SimpleLogger<O> {
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        let module = metadata.target();

        let level = self
            .sorted_module_levels
            .iter()
            .find(|(item, _level)| module.starts_with(item))
            .map(|(_item, level)| level)
            .unwrap_or(&self.max_level);

        &self.max_level >= level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{}", record.args());

            let mut item = String::new();

            for prop in &self.default_props {
                item = format!("{} {}={}", item, prop.0, prop.1.simple_value(record));
            }

            item = format!("{} {}=\"{}\"", item, self.msg_prop, msg);

            writeln!(self.output.lock(), "{}", item.trim())
                .expect("Unable to write to logger output");
        }
    }
}

impl StaticProperty {
    pub fn simple_value(&self) -> String {
        match self {
            Self::String(v) => format!("\"{}\"", v.to_string()),
            Self::Number(v) => format!("{}", v),
            Self::Null => String::new(),
        }
    }
}

impl Property {
    pub fn simple_value(&self, record: &Record) -> String {
        match self {
            Self::Static(prop) => prop.simple_value(),
            Self::Function(f) => f(record).simple_value(),
        }
    }
}
