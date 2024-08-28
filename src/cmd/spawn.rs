use super::err::Error;
use super::handle::Handle;
use super::io::{ChildStdin, ChildStdout};

pub trait Spawn<T>: Sized {
    fn spawn(self) -> Result<T, Error>;
}

pub trait WriteSpawn<T>: Spawn<T> {
    fn write_spawn(self) -> Result<(ChildStdin, T), Error>;
}

pub trait ReadSpawn<T>: Spawn<T> {
    fn read_spawn(self) -> Result<(ChildStdout, T), Error>;
}

pub trait ReadSpawnExt<T>: ReadSpawn<T> {
    fn read_to_vec(self) -> Result<Vec<u8>, Error>;
    fn read_to_string(self) -> Result<String, Error>;
}

pub trait WriteReadSpawn: Spawn<Handle> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error>;
}
