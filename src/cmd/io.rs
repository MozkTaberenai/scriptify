#[derive(Debug)]
pub struct Inherit;

#[derive(Debug)]
pub struct Piped;

use std::io::{Read, Write};

#[derive(Debug)]
pub struct ChildStdin(pub(crate) std::process::ChildStdin);

impl Write for ChildStdin {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

#[derive(Debug)]
pub struct ChildStdout(pub(crate) std::process::ChildStdout);

impl Read for ChildStdout {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl ChildStdout {
    pub fn read_to_end(&mut self) -> std::io::Result<Vec<u8>> {
        let mut vec = vec![];
        self.0.read_to_end(&mut vec)?;
        Ok(vec)
    }

    pub fn read_to_string(&mut self) -> std::io::Result<String> {
        let mut string = String::new();
        self.0.read_to_string(&mut string)?;
        Ok(string)
    }
}
