tacit
=====
An obvious logging library for the rust [log](https://crates.io/crates/log) ecosystem.

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
