use std::io::{stderr, stdout, Write};

#[derive()]
pub struct Writer {
    pub use_stderr: bool,
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.use_stderr {
            true => stderr().write(buf),
            false => stdout().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.use_stderr {
            true => stderr().flush(),
            false => stdout().flush(),
        }
    }
}
