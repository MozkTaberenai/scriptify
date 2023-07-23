use super::*;
use crate::*;

#[derive(Debug)]
pub struct Inherit;

#[derive(Debug)]
pub struct Piped;

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
            write!(f, " {} {}", "|".magenta(), command)?;
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
        let mut echo = echo(self.quiet);
        echo.put(&self);
        echo.end();
        self.inner_spawn(false, false)
    }
}

impl WriteSpawn<Handle> for Pipeline<Inherit, Inherit> {
    fn write_spawn(self) -> Result<(PipeStdin, Handle), Error> {
        self.pipe_stdin().write_spawn()
    }
}

impl ReadSpawn<Handle> for Pipeline<Inherit, Inherit> {
    fn read_spawn(self) -> Result<(PipeStdout, Handle), Error> {
        self.pipe_stdout().read_spawn()
    }
}

impl WriteReadSpawn for Pipeline<Inherit, Inherit> {
    fn write_read_spawn(self) -> Result<(PipeStdin, PipeStdout, Handle), Error> {
        self.pipe_stdio().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Piped, Inherit> {
    fn spawn(mut self) -> Result<Handle, Error> {
        let mut echo = echo(self.quiet);
        echo.put("─▶|".magenta());
        echo.put(&self);
        echo.end();
        self.inner_spawn(true, false)
    }
}

impl WriteSpawn<Handle> for Pipeline<Piped, Inherit> {
    fn write_spawn(self) -> Result<(PipeStdin, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        Ok((stdin, handle))
    }
}

impl WriteReadSpawn for Pipeline<Piped, Inherit> {
    fn write_read_spawn(self) -> Result<(PipeStdin, PipeStdout, Handle), Error> {
        self.pipe_stdout().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Inherit, Piped> {
    fn spawn(mut self) -> Result<Handle, Error> {
        let mut echo = echo(self.quiet);
        echo.put(&self);
        echo.put("|─▶".magenta());
        echo.end();
        self.inner_spawn(false, true)
    }
}

impl ReadSpawn<Handle> for Pipeline<Inherit, Piped> {
    fn read_spawn(self) -> Result<(PipeStdout, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdout = handle.take_stdout().unwrap();
        Ok((stdout, handle))
    }
}

impl WriteReadSpawn for Pipeline<Inherit, Piped> {
    fn write_read_spawn(self) -> Result<(PipeStdin, PipeStdout, Handle), Error> {
        self.pipe_stdin().write_read_spawn()
    }
}

impl Spawn<Handle> for Pipeline<Piped, Piped> {
    fn spawn(mut self) -> Result<Handle, Error> {
        let mut echo = echo(self.quiet);
        echo.put("─▶|".magenta());
        echo.put(&self);
        echo.put("|─▶".magenta());
        echo.end();
        self.inner_spawn(true, true)
    }
}

impl WriteReadSpawn for Pipeline<Piped, Piped> {
    fn write_read_spawn(self) -> Result<(PipeStdin, PipeStdout, Handle), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        let stdout = handle.take_stdout().unwrap();
        Ok((stdin, stdout, handle))
    }
}
