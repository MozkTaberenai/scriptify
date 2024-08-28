#[derive(Debug)]
pub struct Inherit;

#[derive(Debug)]
pub struct Piped;

use std::io::{Read, Write};

/// A handle to the standard input of a child process.
/// It implements the Write trait.
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

/// A handle to the standard output of a child process.
/// It implements the Read trait.
#[derive(Debug)]
pub struct ChildStdout(pub(crate) std::process::ChildStdout);

impl Read for ChildStdout {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
