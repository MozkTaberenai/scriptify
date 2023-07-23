use super::*;
use crate::*;

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
                "{}{}",
                "cd:".bright_blue(),
                current_dir.to_string_lossy().underline().bright_blue(),
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
                    write!(
                        f,
                        "{}{}{}{}",
                        "env:".bright_blue(),
                        k.to_string_lossy().underline().bright_blue(),
                        "=".bright_black(),
                        v.to_string_lossy().underline().bright_blue(),
                    )?;
                } else {
                    write!(
                        f,
                        "{}{}{}",
                        "env:".bright_blue(),
                        "!".bright_blue(),
                        k.to_string_lossy().underline().bright_blue(),
                    )?;
                }
                somethin_written = true;
            }
        }

        if somethin_written {
            write!(f, " ")?;
        }
        write!(
            f,
            "{}",
            self.inner.get_program().to_string_lossy().bold().cyan()
        )?;

        for arg in self.inner.get_args() {
            write!(f, " {}", arg.to_string_lossy().underline().bold())?;
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
    fn read_spawn(self) -> Result<(PipeStdout, Handle), Error> {
        Pipeline::from(self).pipe_stdout().read_spawn()
    }
}

impl WriteSpawn<Handle> for Command {
    fn write_spawn(self) -> Result<(PipeStdin, Handle), Error> {
        Pipeline::from(self).pipe_stdin().write_spawn()
    }
}

impl WriteReadSpawn for Command {
    fn write_read_spawn(self) -> Result<(PipeStdin, PipeStdout, Handle), Error> {
        Pipeline::from(self).pipe_stdio().write_read_spawn()
    }
}
