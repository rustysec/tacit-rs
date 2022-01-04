//! # Property
//! Properties represent an element in the logging structure.
//! Log entries are made up of a series of properties.

use log::Record;

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

impl<T: Into<StaticProperty>> From<Option<T>> for StaticProperty {
    fn from(input: Option<T>) -> Self {
        match input {
            None => Self::Null,
            Some(value) => value.into(),
        }
    }
}

/// The two types of properties a Logger can use, Static or Function.
pub enum Property {
    /// Represent a value that can be included in a log entry
    Static(StaticProperty),
    /// Represent a dynamic function that returns a StaticProperty
    Function(Box<dyn Fn(&Record) -> StaticProperty + Send + Sync>),
}
