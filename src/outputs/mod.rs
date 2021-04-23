//! # Outputs
//! Outputs tell `tacit` where to send the logged information.
//! Examples include the console, a file, a database, etc.

mod simple_console_output;

pub use simple_console_output::*;
use std::io::Write;

/// Defines output implementations
pub trait TacitOutput: Default + Write + Send + Sync {}
