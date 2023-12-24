use std::io::{stderr, stdout, Write};

use crate::Config;
#[derive(Clone, Copy)]
/// Override Output stream
pub struct Writer {
    /// use stderr instead of stdout
    use_stderr: bool,
}

impl Writer {
    /// create new instance of writer that outputs to stderr
    pub fn stderr() -> Writer {
        Writer { use_stderr: true }
    }
    /// create new instance of writer that outputs to stdout
    pub fn stdout() -> Writer {
        Writer { use_stderr: false }
    }
    /// Create new instance of writer or use overwritten writer from config
    ///
    /// always prefers override_writer over `use_stderr` flag
    pub(crate) fn from_config(config: &Config) -> Writer {
        match (config.override_writer, config.use_stderr) {
            (Some(writer), _) => writer,
            (None, use_stderr) => Writer {
                use_stderr: use_stderr,
            },
        }
    }
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
