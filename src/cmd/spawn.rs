use super::err::Error;
use super::handle::Handle;
use super::io::{ChildStdin, ChildStdout};

/// A trait for spawning a process.
pub trait Spawn<T>: Sized {
    fn spawn(self) -> Result<T, Error>;
}

/// A trait for spawning a process with a write handle.
pub trait WriteSpawn<T>: Spawn<T> {
    fn write_spawn(self) -> Result<(ChildStdin, T), Error>;
}

/// A trait for spawning a process with a read handle.
pub trait ReadSpawn<T>: Spawn<T> {
    fn read_spawn(self) -> Result<(ChildStdout, T), Error>;
}

/// Extension methods for `ReadSpawn`.
pub trait ReadSpawnExt<T>: ReadSpawn<T> {
    fn read_to_vec(self) -> Result<Vec<u8>, Error>;
    fn read_to_string(self) -> Result<String, Error>;
}

/// A trait for spawning a process with a write and read handle.
pub trait WriteReadSpawn: Spawn<Handle> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error>;
}
