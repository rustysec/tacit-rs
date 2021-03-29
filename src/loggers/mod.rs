//! # Loggers
//! Loggers define how the output is formatted or structured.

#[cfg(feature = "json")]
mod json_logger;
mod simple_logger;

#[cfg(feature = "json")]
pub use json_logger::*;
pub use simple_logger::*;
