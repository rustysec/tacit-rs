//! # Json Logger
//! Structures log output in json format.
//!
//! ```json
//! {"level":"INFO","msg":"logging a thing","timeStamp":"2021-03-29T15:49:16.425441203+00:00"}
//! ```

use crate::{Logger, Property, SimpleConsoleOutput, StaticProperty, TacitOutput};
use log::{LevelFilter, Metadata, Record};
use parking_lot::Mutex;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};

pub struct JsonLogger<O: TacitOutput> {
    output: Arc<Mutex<O>>,
    msg_prop: String,
    default_props: Vec<(String, Property)>,
    max_level: LevelFilter,
    module_levels: HashMap<String, LevelFilter>,
    sorted_module_levels: Vec<(String, LevelFilter)>,
    explicit: bool,
}

impl Default for JsonLogger<SimpleConsoleOutput> {
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

impl<O: TacitOutput> JsonLogger<O> {
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

impl<O: TacitOutput> crate::Logger for JsonLogger<O> {
    fn level_filter(&self) -> log::LevelFilter {
        if self.explicit {
            log::LevelFilter::Trace
        } else {
            self.max_level
        }
    }

    fn set_level_filter(&mut self, level: log::LevelFilter) {
        self.max_level = level;
    }

    fn with_level_filter(mut self, level: log::LevelFilter) -> Self {
        self.set_level_filter(level);
        self
    }

    fn set_module_level_filter(&mut self, module: String, level: log::LevelFilter) {
        self.module_levels.insert(module, level);
    }

    fn with_module_level_filter(mut self, module: String, level: log::LevelFilter) -> Self {
        self.set_module_level_filter(module, level);
        self
    }

    fn add_fn_prop(&mut self, name: String, prop: fn(&Record) -> StaticProperty) {
        self.default_props
            .push((name, Property::Function(Box::new(prop))));
    }

    fn with_fn_prop(mut self, name: String, prop: fn(&Record) -> StaticProperty) -> Self {
        self.add_fn_prop(name, prop);
        self
    }

    fn add_prop(&mut self, name: String, prop: StaticProperty) {
        self.default_props.push((name, Property::Static(prop)));
    }

    fn with_prop(mut self, name: String, prop: StaticProperty) -> Self {
        self.add_prop(name, prop);
        self
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

impl<O: TacitOutput> log::Log for JsonLogger<O> {
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        let module = metadata.target();

        let level = self
            .sorted_module_levels
            .iter()
            .find(|(item, _level)| module.starts_with(item))
            .map(|(_item, level)| level)
            .unwrap_or(&self.max_level);

        if self.explicit {
            &LevelFilter::Off != level
        } else {
            &self.max_level <= level
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let line = format!("{}", record.args());

            let mut item = json!({ self.msg_prop.clone(): line });

            for prop in &self.default_props {
                item[prop.0.clone()] = prop.1.json_value(record);
            }

            writeln!(
                self.output.lock(),
                "{}",
                serde_json::to_string(&item).expect("Generated invalid JSON object during logging")
            )
            .expect("Unable to write to logger output");
        }
    }
}

impl StaticProperty {
    pub fn json_value(&self) -> Value {
        match self {
            Self::String(v) => Value::String(v.to_string()),
            Self::Number(v) => Value::Number((*v).into()),
            Self::Null => Value::Null,
        }
    }
}

impl Property {
    pub fn json_value(&self, record: &Record) -> Value {
        match self {
            Self::Static(prop) => prop.json_value(),
            Self::Function(f) => f(record).json_value(),
        }
    }
}
