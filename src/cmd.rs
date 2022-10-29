use super::*;

use std::process::{Child, ChildStdout, Command, Output, Stdio};

use once_cell::sync::Lazy;
static ECHO_PREFIX: Lazy<String> = Lazy::new(|| echo::prefix("cmd"));

#[derive(Debug)]
pub enum Error {
    Exit { code: i32 },
    Terminated,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Exit { code } => write!(f, "Exit with error status code: {code}"),
            Error::Terminated => write!(f, "Terminated by signal"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Cmd {
    inner: Command,
    echo: Option<Echo>,
}

impl Cmd {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        let inner = Command::new(program);
        let echo = None;
        Cmd { inner, echo }
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
        match self.echo {
            Some(ref mut echo) => {
                echo.quiet();
            }
            None => {
                let mut echo = Echo::new();
                echo.quiet();
                self.echo.replace(echo);
            }
        }
        self
    }

    fn echo_command_info(&mut self) {
        if self.echo.is_none() {
            let mut echo = Echo::new();
            echo.put(&*ECHO_PREFIX);
            self.echo.replace(echo);
        }

        let inner = &mut self.inner;
        let echo = self.echo.as_mut().unwrap();

        if let Some(current_dir) = inner.get_current_dir() {
            let current_dir = format!(
                "{}{}",
                "cwd:".bright_black(),
                current_dir.to_string_lossy().underline().bright_black(),
            );
            echo.put(current_dir);
        }

        let envs = inner.get_envs();
        if envs.len() > 0 {
            for (k, v) in envs {
                match v {
                    Some(v) => {
                        echo.put(format!(
                            "{}{}{}{}",
                            "env:".bright_black(),
                            k.to_string_lossy().underline().bright_black(),
                            "=".bright_black(),
                            v.to_string_lossy().underline().bright_black(),
                        ));
                    }
                    None => {
                        echo.put(format!(
                            "{}{}{}",
                            "env:".bright_black(),
                            "!".bright_black(),
                            k.to_string_lossy().underline().bright_black(),
                        ));
                    }
                }
            }
        }

        echo.put(
            inner
                .get_program()
                .to_string_lossy()
                .bold()
                .cyan()
                .to_string(),
        );

        for arg in inner.get_args() {
            echo.put(arg.to_string_lossy().underline().bold().to_string());
        }
    }

    pub fn run(mut self) -> Result<()> {
        self.echo_command_info();

        self.echo.unwrap().end();

        let status = self.inner.status().echo_err()?;

        match status.success() {
            true => Ok(()),
            false => {
                let err = match status.code() {
                    Some(code) => Error::Exit { code },
                    None => Error::Terminated,
                };
                Err(err).echo_err()?
            }
        }
    }

    pub fn output(mut self) -> Result<Output> {
        self.echo_command_info();
        let mut echo = self.echo.take().unwrap();

        match self.inner.output() {
            Err(err) => {
                echo.end();
                Err(err).echo_err()?
            }
            Ok(output) => {
                echo.put("| output".magenta());
                if !output.stdout.is_empty() {
                    echo.put(format!("stdout: {} bytes", output.stdout.len()).magenta());
                }
                if !output.stderr.is_empty() {
                    echo.put(format!("stderr: {} bytes", output.stderr.len()).magenta());
                }
                echo.end();
                Ok(output)
            }
        }
    }

    pub fn spawn(mut self) -> Result<Child> {
        self.echo_command_info();
        self.echo.unwrap().end();
        Ok(self.inner.spawn().echo_err()?)
    }

    pub fn into_inner(mut self) -> Command {
        self.echo_command_info();
        self.echo.unwrap().end();
        self.inner
    }

    pub fn pipe(mut self) -> Result<Pipe> {
        self.echo_command_info();
        self.inner.stdout(Stdio::piped());
        let mut child = self.inner.spawn().echo_err()?;
        let stdout = child.stdout.take().unwrap();
        let echo = self.echo.take().unwrap();
        Ok(Pipe { stdout, echo })
    }
}

pub struct Pipe {
    stdout: ChildStdout,
    echo: Echo,
}

impl Pipe {
    pub fn into_reader(mut self) -> ChildStdout {
        self.echo.put("| into_reader".magenta());
        self.echo.end();
        self.stdout
    }

    pub fn into_cmd(mut self, mut cmd: Cmd) -> Cmd {
        self.echo.put("|".magenta());
        cmd.echo.replace(self.echo);
        cmd.inner.stdin(Stdio::from(self.stdout));
        cmd
    }
}

#[macro_export]
macro_rules! cmd {
    ($program:expr) => {
        Cmd::new($program)
    };
    ($program:expr, $($arg:expr),* $(,)?) => {
        Cmd::new($program)$(.arg($arg))*
    };
}
