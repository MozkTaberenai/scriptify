use super::child::Child;
use super::err::Error;
use super::handle::Handle;
use super::io::{ChildStdin, ChildStdout, Inherit};
use super::pipeline::{Pipe, Pipeline};
use super::spawn::*;
use super::status::Status;
use crate::style::{
    BOLD_CYAN, BOLD_UNDERLINE, BRIGHT_BLACK, BRIGHT_BLUE, RESET, UNDERLINE, UNDERLINE_BRIGHT_BLUE,
};
use std::ffi::OsStr;
use std::path::Path;

/// A command pipeline builder.
///
/// This struct represent a sigle command.
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
    /// Creates a new `Command` instance with the specified program.
    ///
    /// Argument `program` is passed to `std::process::Command::new`.
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Command {
            inner: std::process::Command::new(program),
            quiet: false,
        }
    }

    /// Returns a reference to the inner `std::process::Command`.
    ///
    /// # Returns
    ///
    /// A reference to the inner `std::process::Command`.
    pub fn get_inner_ref(&self) -> &std::process::Command {
        &self.inner
    }

    /// Returns a mutable reference to the inner `std::process::Command`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the inner `std::process::Command`.
    pub fn get_inner_mut(&mut self) -> &mut std::process::Command {
        &mut self.inner
    }

    /// Consumes the `Command`, returning the inner `std::process::Command`.
    ///
    /// # Returns
    ///
    /// The inner `std::process::Command`.
    pub fn into_inner(self) -> std::process::Command {
        self.inner
    }

    /// Sets the working directory for the child process.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    /// Sets an environment variable for the child process.
    ///
    /// This method bridges to `std::process::Command::env`.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable.
    /// * `val` - The value of the environment variable.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, val);
        self
    }

    /// Clears the entire environment for the child process.
    ///
    /// This method bridges to `std::process::Command::env_clear`.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn env_clear(mut self) -> Self {
        self.inner.env_clear();
        self
    }

    /// Removes an environment variable for the child process.
    ///
    /// This method bridges to `std::process::Command::env_remove`.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable to remove.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn env_remove(mut self, key: impl AsRef<OsStr>) -> Self {
        self.inner.env_remove(key);
        self
    }

    /// Adds an argument to the command.
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument to add.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.inner.arg(arg);
        self
    }

    /// Adds multiple arguments to the command.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator of arguments to add.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn args(mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Self {
        self.inner.args(args);
        self
    }

    /// Sets the command to run quietly (suppresses output).
    ///
    /// # Returns
    ///
    /// Returns `self` to allow for method chaining.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Runs the command and waits for it to complete.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Status` of the completed command, or an `Error` if the command fails.
    pub fn run(self) -> Result<Status, Error> {
        self.spawn()?.wait()
    }

    pub(crate) fn _spawn(mut self) -> Result<Child, Error> {
        Ok(Child {
            std_child: self.inner.spawn().map_err(|source| Error {
                about: Some(self.to_string()),
                source,
            })?,
            command: self,
        })
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
        Pipeline::from(self).pipe_stdout().read_spawn()
    }
}

impl WriteSpawn<Handle> for Command {
    fn write_spawn(self) -> Result<(ChildStdin, Handle), Error> {
        Pipeline::from(self).pipe_stdin().write_spawn()
    }
}

impl WriteReadSpawn for Command {
    fn write_read_spawn(self) -> Result<(ChildStdin, ChildStdout, Handle), Error> {
        Pipeline::from(self).pipe_stdio().write_read_spawn()
    }
}
