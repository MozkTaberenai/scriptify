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
    fn read_to_end(self) -> Result<Vec<u8>, Error>;
    fn read_to_string(self) -> Result<String, Error>;
}

pub trait WriteReadSpawn: Spawn<Handle> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error>;
}

impl<T> ReadSpawnExt<Handle> for T
where
    T: ReadSpawn<Handle>,
{
    fn read_to_end(self) -> Result<Vec<u8>, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let vec = stdout.read_to_end().map_err(|source| Error {
            on: Some(handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(vec)
    }

    fn read_to_string(self) -> Result<String, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let string = stdout.read_to_string().map_err(|source| Error {
            on: Some(handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(string)
    }
}
