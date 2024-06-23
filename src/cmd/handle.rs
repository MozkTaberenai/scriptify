use super::child::Child;
use super::err::Error;
use super::io::{ChildStdin, ChildStdout};
use super::status::Status;

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
