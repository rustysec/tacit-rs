//! # Simple Formatter
//! Structures log output in a simple parsable format.
//!
//! ```sh
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
        ignore_empty_props: bool,
    ) where
        O: TacitOutput,
    {
        let msg = format!("{}", record.args());

        let mut item = String::new();

        for prop in default_props {
            let value = prop.1.simple_value(record);
            if ignore_empty_props && value.is_empty() {
                continue;
            }
            item = format!("{} {}={}", item, prop.0, value);
        }

        item = format!("{} {}=\"{}\"", item, msg_prop, msg);

        writeln!(output, "{}", item.trim()).expect("Unable to write to logger output");
    }
}

impl StaticProperty {
    pub fn simple_value(&self) -> String {
        match self {
            Self::String(v) => format!("\"{}\"", v),
            Self::Number(v) => v.to_string(),
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
