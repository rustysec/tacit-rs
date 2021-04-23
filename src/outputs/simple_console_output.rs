//! # Simple Console Output
//! Write all output to the console (stdout).

use super::TacitOutput;
use std::io::{stdout, Write};

#[derive(Default)]
pub struct SimpleConsoleOutput {}

impl TacitOutput for SimpleConsoleOutput {}

impl Write for SimpleConsoleOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut stdout = stdout();
        stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout.flush()
    }
}
