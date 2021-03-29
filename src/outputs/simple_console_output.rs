use super::TacitOutput;
use parking_lot::Mutex;
use std::{
    io::{stdout, Write},
    sync::Arc,
};

#[derive(Default)]
pub struct SimpleConsoleOutput {
    guard: Arc<Mutex<()>>,
}

impl TacitOutput for SimpleConsoleOutput {}

impl Write for SimpleConsoleOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let _lock = self.guard.lock();
        let mut stdout = stdout();
        stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let _lock = self.guard.lock();
        let mut stdout = stdout();
        stdout.flush()
    }
}
