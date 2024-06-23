use super::child::Child;
use super::err::Error;
use super::handle::Handle;
use super::io::{ChildStdin, ChildStdout, Inherit, Piped};
use super::pipeline::{Pipe, Pipeline};
use super::spawn::*;
use super::status::Status;
use crate::{style, Style};
use std::ffi::OsStr;
use std::path::Path;

const BRIGHT_BLUE: Style = style().bright_blue();
const BRIGHT_BLACK: Style = style().bright_black();
const UNDERLINE: Style = style().underline();
const BOLD_UNDERLINE: Style = style().bold().underline();
const UNDERLINE_BRIGHT_BLUE: Style = style().underline().bright_blue();
const BOLD_CYAN: Style = style().bold().cyan();
const RESET: anstyle::Reset = anstyle::Reset;

#[derive(Debug)]
pub struct Command {
    pub(crate) inner: std::process::Command,
    pub(crate) quiet: bool,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut somethin_written = false;

        if let Some(current_dir) = self.inner.get_current_dir() {
            write!(
                f,
                "{BRIGHT_BLUE}cd:{UNDERLINE}{}{RESET}",
                current_dir.to_string_lossy()
            )?;
            somethin_written = true;
        }

        let envs = self.inner.get_envs();
        if envs.len() > 0 {
            for (k, v) in envs {
                if somethin_written {
                    write!(f, " ")?;
                }
                if let Some(v) = v {
                    write!(f, "{BRIGHT_BLUE}env:{RESET}")?;
                    write!(f, "{BRIGHT_BLUE}{UNDERLINE}{}{RESET}", k.to_string_lossy())?;
                    write!(f, "{BRIGHT_BLACK}={RESET}")?;
                    write!(f, "{UNDERLINE_BRIGHT_BLUE}{}{RESET}", v.to_string_lossy())?;
                } else {
                    write!(f, "{BRIGHT_BLUE}env:!{RESET}")?;
                    write!(f, "{UNDERLINE_BRIGHT_BLUE}{}{RESET}", k.to_string_lossy())?;
                }
                somethin_written = true;
            }
        }

        if somethin_written {
            write!(f, " ")?;
        }
        write!(
            f,
            "{BOLD_CYAN}{}{RESET}",
            self.inner.get_program().to_string_lossy()
        )?;

        for arg in self.inner.get_args() {
            write!(f, " {BOLD_UNDERLINE}{}{RESET}", arg.to_string_lossy())?;
        }

        Ok(())
    }
}

impl Command {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Command {
            inner: std::process::Command::new(program),
            quiet: false,
        }
    }

    pub fn get_inner_ref(&self) -> &std::process::Command {
        &self.inner
    }

    pub fn get_inner_mut(&mut self) -> &mut std::process::Command {
        &mut self.inner
    }

    pub fn into_inner(self) -> std::process::Command {
        self.inner
    }

    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, val);
        self
    }

    pub fn env_clear(mut self) -> Self {
        self.inner.env_clear();
        self
    }

    pub fn env_remove(mut self, key: impl AsRef<OsStr>) -> Self {
        self.inner.env_remove(key);
        self
    }

    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.inner.arg(arg);
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Self {
        self.inner.args(args);
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    pub(crate) fn inner_spawn(mut self) -> Result<Child, Error> {
        Ok(Child {
            std_child: self.inner.spawn().map_err(|source| Error {
                on: Some(self.to_string()),
                source,
            })?,
            command: self,
        })
    }

    pub fn run(self) -> Result<Status, Error> {
        self.spawn()?.wait()
    }

    pub fn pipe_stdio(self) -> Pipeline<Piped, Piped> {
        Pipeline::from(self).pipe_stdio()
    }

    pub fn pipe_stdin(self) -> Pipeline<Piped, Inherit> {
        Pipeline::from(self).pipe_stdin()
    }

    pub fn pipe_stdout(self) -> Pipeline<Inherit, Piped> {
        Pipeline::from(self).pipe_stdout()
    }
}

impl Pipe<Command> for Command {
    type In = Inherit;
    type Out = Inherit;

    fn pipe(self, cmd: Command) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            quiet: self.quiet,
            inner: vec![self, cmd],
            input: Inherit,
            output: Inherit,
        }
    }
}

impl Spawn<Handle> for Command {
    fn spawn(self) -> Result<Handle, Error> {
        Pipeline::from(self).spawn()
    }
}

impl ReadSpawn<Handle> for Command {
    fn read_spawn(self) -> Result<(ChildStdout, Handle), Error> {
        self.pipe_stdout().read_spawn()
    }
}

impl WriteSpawn<Handle> for Command {
    fn write_spawn(self) -> Result<(ChildStdin, Handle), Error> {
        self.pipe_stdin().write_spawn()
    }
}

impl WriteReadSpawn for Command {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        self.pipe_stdio().write_read_spawn()
    }
}
