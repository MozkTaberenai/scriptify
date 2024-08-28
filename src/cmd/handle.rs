use super::child::Child;
use super::err::Error;
use super::io::{ChildStdin, ChildStdout};
use super::spawn::{ReadSpawn, ReadSpawnExt};
use super::status::Status;
use std::io::Read;

#[derive(Debug, Default)]
pub struct Handle(pub(crate) Vec<Child>);

impl From<Child> for Handle {
    fn from(child: Child) -> Self {
        Handle(vec![child])
    }
}

impl From<Vec<Child>> for Handle {
    fn from(children: Vec<Child>) -> Self {
        Handle(children)
    }
}

impl Handle {
    pub fn wait(self) -> Result<Status, Error> {
        let mut status = Vec::with_capacity(self.0.len());
        for child in self.0 {
            status.push(child.wait()?);
        }
        Ok(Status(status))
    }

    pub(crate) fn take_stdin(&mut self) -> Option<ChildStdin> {
        self.0
            .first_mut()
            .expect("handle has no children")
            .take_stdin()
            .map(ChildStdin)
    }

    pub(crate) fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.0
            .last_mut()
            .expect("handle has no children")
            .take_stdout()
            .map(ChildStdout)
    }
}

impl<RS> ReadSpawnExt<Handle> for RS
where
    RS: ReadSpawn<Handle>,
{
    fn read_to_vec(self) -> Result<Vec<u8>, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let mut vec = Vec::new();
        stdout.read_to_end(&mut vec).map_err(|source| Error {
            about: Some(handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(vec)
    }

    fn read_to_string(self) -> Result<String, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let mut string = String::new();
        stdout.read_to_string(&mut string).map_err(|source| Error {
            about: Some(handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(string)
    }
}

#[derive(Debug)]
pub struct ThreadHandle<T> {
    pub(crate) handle: Handle,
    pub(crate) thread: std::thread::JoinHandle<std::io::Result<T>>,
}

impl<T> ThreadHandle<T> {
    pub(crate) fn take_stdin(&mut self) -> Option<ChildStdin> {
        self.handle.take_stdin()
    }

    pub(crate) fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.handle.take_stdout()
    }

    pub fn wait(self) -> Result<(Status, T), Error> {
        let status = self.handle.wait()?;
        let writer = self.thread.join().expect("fail to join io thread")?;
        Ok((status, writer))
    }
}

impl<T, RS> ReadSpawnExt<ThreadHandle<T>> for RS
where
    RS: ReadSpawn<ThreadHandle<T>>,
{
    fn read_to_vec(self) -> Result<Vec<u8>, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let mut vec = Vec::new();
        stdout.read_to_end(&mut vec).map_err(|source| Error {
            about: Some(handle.handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(vec)
    }

    fn read_to_string(self) -> Result<String, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let mut string = String::new();
        stdout.read_to_string(&mut string).map_err(|source| Error {
            about: Some(handle.handle.0.last().unwrap().command.to_string()),
            source,
        })?;
        handle.wait()?;
        Ok(string)
    }
}
