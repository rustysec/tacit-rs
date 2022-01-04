//! # Json Formatter
//! Structures log output in json format.
//!
//! ```json
//! {"level":"INFO","msg":"logging a thing","timeStamp":"2021-03-29T15:49:16.425441203+00:00"}
//! ```

use crate::{formatters::TacitFormatter, Property, StaticProperty, TacitOutput};
use log::Record;
use serde_json::{json, Value};

#[derive(Default)]
pub struct JsonFormatter {}

impl TacitFormatter for JsonFormatter {
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
        let line = format!("{}", record.args());

        let mut item = json!({ msg_prop: line });

        for prop in default_props {
            let value = prop.1.json_value(record);
            if ignore_empty_props && matches!(value, Value::Null) {
                continue;
            }
            item[prop.0.clone()] = value;
        }

        #[cfg(feature = "kv")]
        {
            let source = record.key_values();
            let mut visitor = super::KvVisitor::new(item);

            for _idx in 0..source.count() {
                if source.visit(&mut visitor).is_err() {
                    break;
                }
            }

            item = visitor.inner();
        }

        writeln!(
            output,
            "{}",
            serde_json::to_string(&item).expect("Generated invalid JSON object during logging")
        )
        .expect("Unable to write to logger output");
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
