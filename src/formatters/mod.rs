//! # Formatters
//! Formatters define how the output is displayed or structured.

#[cfg(feature = "json")]
mod json_formatter;
mod simple_formatter;

#[cfg(feature = "json")]
pub use json_formatter::*;
pub use simple_formatter::*;

use crate::{Property, TacitOutput};
use log::Record;

pub trait TacitFormatter: Default + Send + Sync {
    fn log<O>(
        &self,
        output: &mut O,
        record: &Record,
        msg_prop: &str,
        default_props: &[(String, Property)],
        ignore_empty_props: bool,
    ) where
        O: TacitOutput;
}

#[cfg(feature = "kv")]
pub struct KvVisitor(serde_json::Value);

#[cfg(feature = "kv")]
impl KvVisitor {
    pub fn new(item: serde_json::Value) -> Self {
        Self(item)
    }

    pub fn inner(self) -> serde_json::Value {
        self.0
    }
}

#[cfg(feature = "kv")]
impl<'kvs> log::kv::Visitor<'kvs> for KvVisitor {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        let result = if let Some(value) = value.to_borrowed_str() {
            serde_json::Value::String(value.to_string())
        } else if let Some(value) = value.to_bool() {
            serde_json::Value::Bool(value)
        } else if let Some(value) = value.to_i64() {
            serde_json::Value::Number(serde_json::Number::from(value))
        } else if let Some(value) = value.to_f64() {
            serde_json::Number::from_f64(value)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        };

        self.0[key.to_string()] = result;

        Ok(())
    }
}
