tacit
=====
![Build Status](https://github.com/rustysec/tacit-rs/workflows/Build/badge.svg)

An obvious logging library for Rust's [log](https://crates.io/crates/log) ecosystem.

## Overview
There are a lot of great and very useful logging libraries for Rust.
However, some are a little too simple, others a little too complex.
`tacit` aims to walk the fine line with enough features to provide
the power most users would want, but with a simple and obvious
interface that makes discovering "how to use this" easy.


## Principals
There are two main components that make up the `tacit` logging system.
These are [formatters](#formatters) and [outputs](#outputs).


### Formatters 
Loggers control the output format of the entries. This can be a simple
line of text, or something more structured like JSON or CEF.


### Outputs
Outputs dictate where the log entries arrive. Examples include the console,
a file, an archive, a database, etc.


## Usage
`tacit` has a very simple API surface, meant to provide enough options to be 
usefully without overwhelming developers. 

[Formatters](#formatters) and [Outputs](#outputs) both implement `Default` so
you can be reasonably assured that logging will work with sane defaults. A
simple setup involves something like this:

```rust
use tacit::{JsonFormatter, Logger, SimpleConsoleOutput};

let json_logger = Logger::<SimpleConsoleOutput, JsonFormatter>::default();
tacit::new().with_logger(json_logger).log().unwrap();
log::info!("logging some info!");
```

In the event that a [formatter](#formatters) or [output](#outputs) has specific
configuration options, they can be used like this:

```rust
use tacit::{JsonFormatter, Logger, SimpleConsoleOutput};

let output = SimpleConsoleOutput::default(); // with options...
let formatter = JsonFormatter::default(); // with options...
let json_logger = Logger::new(output, formatter);
tacit::new().with_logger(json_logger).log().unwrap();
log::info!("logging some info!");
```

