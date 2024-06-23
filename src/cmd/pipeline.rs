use super::command::Command;
use super::err::Error;
use super::handle::{Handle, ThreadHandle};
use super::io::{ChildStdin, ChildStdout, Inherit, Piped};
use super::spawn::*;
use super::status::Status;
use crate::{style, Echo, Style};

const BRIGHT_BLACK: Style = style().bright_black();
const MAGENTA: Style = style().magenta();
const RESET: anstyle::Reset = anstyle::Reset;

fn echo(quiet: bool) -> Echo {
    match quiet {
        true => Echo::quiet(),
        false => Echo::new(),
    }
    .sput("cmd", BRIGHT_BLACK)
}

pub trait Pipe<I> {
    type In;
    type Out;
    fn pipe(self, to: I) -> Pipeline<Self::In, Self::Out>;
}

#[derive(Debug)]
pub struct Pipeline<I, O> {
    pub(crate) inner: Vec<Command>,
    pub(crate) input: I,
    pub(crate) output: O,
    pub(crate) quiet: bool,
}

impl From<Command> for Pipeline<Inherit, Inherit> {
    fn from(command: Command) -> Self {
        Self {
            input: Inherit,
            output: Inherit,
            quiet: command.quiet,
            inner: vec![command],
        }
    }
}

impl<I, O> std::fmt::Display for Pipeline<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter();
        let first = iter.next().unwrap();
        write!(f, "{}", first)?;
        for command in iter {
            write!(f, " {MAGENTA}|{RESET} {}", command)?;
        }
        Ok(())
    }
}

impl<I, O> Pipeline<I, O> {
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    pub(crate) fn inner_spawn(&mut self, pipein: bool, pipeout: bool) -> Result<Handle, Error> {
        use std::process::Stdio;

        let mut stdin_config = match pipein {
            true => Some(Stdio::piped()),
            false => None,
        };

        let mut children = Vec::with_capacity(self.inner.len());
        let mut iter = self.inner.drain(..);
        while let Some(mut command) = iter.next() {
            if let Some(stdout) = stdin_config.take() {
                command.inner.stdin(stdout);
            }
            let remaining = iter.len() > 0;
            if remaining || pipeout {
                command.inner.stdout(Stdio::piped());
            }
            let mut child = command.inner_spawn()?;
            if remaining {
                stdin_config = child.take_stdout().map(Stdio::from);
            }
            children.push(child);
        }

        Ok(Handle::from(children))
    }
}

impl Pipeline<Inherit, Inherit> {
    pub fn run(self) -> Result<Status, Error> {
        self.spawn()?.wait()
    }

    pub fn pipe_stdio(self) -> Pipeline<Piped, Piped> {
        Pipeline {
            inner: self.inner,
            input: Piped,
            output: Piped,
            quiet: self.quiet,
        }
    }
}

impl<O> Pipeline<Inherit, O> {
    pub fn pipe_stdin(self) -> Pipeline<Piped, O> {
        Pipeline {
            inner: self.inner,
            input: Piped,
            output: self.output,
            quiet: self.quiet,
        }
    }
}

impl<I> Pipeline<I, Inherit> {
    pub fn pipe_stdout(self) -> Pipeline<I, Piped> {
        Pipeline {
            inner: self.inner,
            input: self.input,
            output: Piped,
            quiet: self.quiet,
        }
    }
}

impl<I, O> Pipe<Command> for Pipeline<I, O> {
    type In = I;
    type Out = O;

    fn pipe(mut self, cmd: Command) -> Pipeline<Self::In, Self::Out> {
        self.inner.push(cmd);
        self
    }
}

impl Spawn<Handle> for Pipeline<Inherit, Inherit> {
    fn spawn(mut self) -> Result<Handle, Error> {
        echo(self.quiet).put(&self).end();
        self.inner_spawn(false, false)
    }
}

impl WriteSpawn<Handle> for Pipeline<Inherit, Inherit> {
    fn write_spawn(self) -> Result<(ChildStdin, Handle), Error> {
        self.pipe_stdin().write_spawn()
    }
}

impl ReadSpawn<Handle> for Pipeline<Inherit, Inherit> {
    fn read_spawn(self) -> Result<(ChildStdout, Handle), Error> {
        self.pipe_stdout().read_spawn()
    }
}

impl WriteReadSpawn for Pipeline<Inherit, Inherit> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        self.pipe_stdio().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Piped, Inherit> {
    fn spawn(mut self) -> Result<Handle, Error> {
        echo(self.quiet).sput("─▶|", MAGENTA).put(&self).end();
        self.inner_spawn(true, false)
    }
}

impl WriteSpawn<Handle> for Pipeline<Piped, Inherit> {
    fn write_spawn(self) -> Result<(ChildStdin, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        Ok((stdin, handle))
    }
}

impl WriteReadSpawn for Pipeline<Piped, Inherit> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        self.pipe_stdout().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Inherit, Piped> {
    fn spawn(mut self) -> Result<Handle, Error> {
        echo(self.quiet).put(&self).sput("|─▶", MAGENTA).end();
        self.inner_spawn(false, true)
    }
}

impl ReadSpawn<Handle> for Pipeline<Inherit, Piped> {
    fn read_spawn(self) -> Result<(ChildStdout, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdout = handle.take_stdout().unwrap();
        Ok((stdout, handle))
    }
}

impl WriteReadSpawn for Pipeline<Inherit, Piped> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        self.pipe_stdin().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Piped, Piped> {
    fn spawn(mut self) -> Result<Handle, Error> {
        echo(self.quiet)
            .sput("─▶|", MAGENTA)
            .put(&self)
            .sput("|─▶", MAGENTA)
            .end();
        self.inner_spawn(true, true)
    }
}

impl WriteReadSpawn for Pipeline<Piped, Piped> {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        let stdout = handle.take_stdout().unwrap();
        Ok((stdin, stdout, handle))
    }
}

// Reader

impl<R> Pipe<Command> for R
where
    R: std::io::Read + Sized,
{
    type In = Self;
    type Out = Inherit;

    fn pipe(self, to: Command) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            inner: vec![to],
            input: self,
            output: Inherit,
            quiet: false,
        }
    }
}

impl<R: std::io::Read + Send + 'static> Spawn<ThreadHandle<()>> for Pipeline<R, Inherit> {
    fn spawn(mut self) -> Result<ThreadHandle<()>, Error> {
        echo(self.quiet).sput("─▶|", MAGENTA).put(&self).end();

        let mut handle = self.inner_spawn(true, false)?;

        let mut reader = self.input;
        let mut writer = handle.take_stdin().unwrap();
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(())
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<R: std::io::Read + Send + 'static> Pipeline<R, Inherit> {
    pub fn run(self) -> Result<Status, Error> {
        self.spawn()?.wait().map(|x| x.0)
    }
}

impl<R: std::io::Read + Send + 'static> ReadSpawn<ThreadHandle<()>> for Pipeline<R, Inherit> {
    fn read_spawn(self) -> Result<(ChildStdout, ThreadHandle<()>), Error> {
        self.pipe_stdout().read_spawn()
    }
}

impl<R: std::io::Read + Send + 'static> Spawn<ThreadHandle<()>> for Pipeline<R, Piped> {
    fn spawn(mut self) -> Result<ThreadHandle<()>, Error> {
        echo(self.quiet)
            .sput("─▶|", MAGENTA)
            .put(&self)
            .sput("|─▶", MAGENTA)
            .end();

        let mut handle = self.inner_spawn(true, true)?;

        let mut reader = self.input;
        let mut writer = handle.take_stdin().unwrap();
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(())
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<R: std::io::Read + Send + 'static> ReadSpawn<ThreadHandle<()>> for Pipeline<R, Piped> {
    fn read_spawn(self) -> Result<(ChildStdout, ThreadHandle<()>), Error> {
        let mut handle = self.spawn()?;
        let stdout = handle.take_stdout().unwrap();
        Ok((stdout, handle))
    }
}

impl<T> ReadSpawnExt<ThreadHandle<()>> for T
where
    T: ReadSpawn<ThreadHandle<()>>,
{
    fn read_to_end(self) -> Result<Vec<u8>, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let vec = stdout.read_to_end()?;
        handle.wait()?;
        Ok(vec)
    }

    fn read_to_string(self) -> Result<String, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let string = stdout.read_to_string()?;
        handle.wait()?;
        Ok(string)
    }
}

// Writer

impl<W> Pipe<W> for Command
where
    W: std::io::Write + Sized,
{
    type In = Inherit;
    type Out = W;

    fn pipe(self, writer: W) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            quiet: self.quiet,
            inner: vec![self],
            input: Inherit,
            output: writer,
        }
    }
}

impl<I, W> Pipe<W> for Pipeline<I, Inherit>
where
    W: std::io::Write + Sized,
{
    type In = I;
    type Out = W;

    fn pipe(self, writer: W) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            inner: self.inner,
            input: self.input,
            output: writer,
            quiet: self.quiet,
        }
    }
}

impl<W: std::io::Write + Send + 'static> Spawn<ThreadHandle<W>> for Pipeline<Inherit, W> {
    fn spawn(mut self) -> Result<ThreadHandle<W>, Error> {
        echo(self.quiet).put(&self).sput("|─▶", MAGENTA).end();

        let mut handle = self.inner_spawn(false, true)?;

        let mut reader = handle.take_stdout().unwrap();
        let mut writer = self.output;
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(writer)
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<W: std::io::Write + Send + 'static> WriteSpawn<ThreadHandle<W>> for Pipeline<Inherit, W> {
    fn write_spawn(self) -> Result<(ChildStdin, ThreadHandle<W>), Error> {
        self.pipe_stdin().write_spawn()
    }
}

impl<W: std::io::Write + Send + 'static> Spawn<ThreadHandle<W>> for Pipeline<Piped, W> {
    fn spawn(mut self) -> Result<ThreadHandle<W>, Error> {
        echo(self.quiet)
            .sput("─▶|", MAGENTA)
            .put(&self)
            .sput("|─▶", MAGENTA)
            .end();

        let mut handle = self.inner_spawn(true, true)?;

        let mut reader = handle.take_stdout().unwrap();
        let mut writer = self.output;
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok::<_, std::io::Error>(writer)
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<W: std::io::Write + Send + 'static> WriteSpawn<ThreadHandle<W>> for Pipeline<Piped, W> {
    fn write_spawn(self) -> Result<(ChildStdin, ThreadHandle<W>), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        Ok((stdin, handle))
    }
}
