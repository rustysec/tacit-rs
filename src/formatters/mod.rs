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
    ) where
        O: TacitOutput;
}
