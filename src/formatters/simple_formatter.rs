//! # Simple Formatter
//! Structures log output in a simple parsable format.
//!
//! ```
//! timeStamp="2021-03-29T15:16:25.683809406+00:00" level="INFO" msg="logging a thing"
//! ```

use crate::{formatters::TacitFormatter, Property, StaticProperty, TacitOutput};
use log::Record;

#[derive(Default)]
pub struct SimpleFormatter {}

impl TacitFormatter for SimpleFormatter {
    fn log<O>(
        &self,
        output: &mut O,
        record: &Record,
        msg_prop: &str,
        default_props: &[(String, Property)],
    ) where
        O: TacitOutput,
    {
        let msg = format!("{}", record.args());

        let mut item = String::new();

        for prop in default_props {
            item = format!("{} {}={}", item, prop.0, prop.1.simple_value(record));
        }

        item = format!("{} {}=\"{}\"", item, msg_prop, msg);

        writeln!(output, "{}", item.trim()).expect("Unable to write to logger output");
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
