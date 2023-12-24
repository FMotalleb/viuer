use std::{
    borrow::Cow,
    io::{stderr, BufWriter, Write},
};

/// Override Output stream
pub struct StringWriter {
    inner_buf: std::io::BufWriter<Vec<u8>>,
}

impl StringWriter {
    pub fn new() -> StringWriter {
        StringWriter {
            inner_buf: BufWriter::new(vec![]),
        }
    }
    pub fn read(&mut self) -> String {
        let result = String::from_utf8_lossy(self.inner_buf.buffer());
        stderr().write(self.inner_buf.buffer());
        return result.to_string();
    }
}

impl Write for StringWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner_buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner_buf.flush()
    }
}
