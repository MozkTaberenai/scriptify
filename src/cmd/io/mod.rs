use std::io::Read;
use std::process::{ChildStdin, ChildStdout};

#[derive(Debug)]
pub struct PipeStdin(pub(crate) ChildStdin);

impl std::io::Write for PipeStdin {
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
pub struct PipeStdout(pub(crate) ChildStdout);

impl std::io::Read for PipeStdout {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl PipeStdout {
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

mod reader;
mod writer;
