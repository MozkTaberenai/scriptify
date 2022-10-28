use std::process::{Child, Command, Output, Stdio};

use super::*;

use once_cell::sync::Lazy;
static ECHO_PREFIX: Lazy<String> = Lazy::new(|| echo::prefix("cmd"));

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Exit with error status code: {code}")]
    Exit { code: i32 },

    #[error("Terminated by signal")]
    Terminated,
}

pub struct Cmd {
    inner: Command,
    quiet: bool,
    piped: bool,
}

impl Cmd {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Cmd {
            inner: Command::new(program),
            quiet: false,
            piped: false,
        }
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

    fn echo(&self) -> Echo {
        let mut echo = Echo::new();

        if self.quiet {
            echo.quiet();
            return echo;
        }

        echo.out(&*ECHO_PREFIX);

        if self.piped {
            echo.out("|".magenta());
        }

        if let Some(current_dir) = self.inner.get_current_dir() {
            let current_dir = format!(
                "{}{}",
                "cwd:".bright_black(),
                current_dir.to_string_lossy().underline().bright_black(),
            );
            echo.out(current_dir);
        }

        let envs = self.inner.get_envs();
        if envs.len() > 0 {
            for (k, v) in envs {
                match v {
                    Some(v) => {
                        echo.out(format!(
                            "{}{}{}{}",
                            "env:".bright_black(),
                            k.to_string_lossy().underline().bright_black(),
                            "=".bright_black(),
                            v.to_string_lossy().underline().bright_black(),
                        ));
                    }
                    None => {
                        echo.out(format!(
                            "{}{}{}",
                            "env:".bright_black(),
                            "!".bright_black(),
                            k.to_string_lossy().underline().bright_black(),
                        ));
                    }
                }
            }
        }

        echo.out(
            self.inner
                .get_program()
                .to_string_lossy()
                .bold()
                .cyan()
                .to_string(),
        );

        for arg in self.inner.get_args() {
            echo.out(arg.to_string_lossy().underline().bold().to_string());
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            program=?self.inner.get_program(),
            args=?self.inner.get_args(),
            current_dir=?self.inner.get_current_dir(),
            envs=?self.inner.get_envs(),
            piped=%self.piped,
        );

        echo
    }

    pub fn run(mut self) -> Result<()> {
        self.echo().end();

        let status = self.inner.status().map_err(echo::error)?;

        match status.success() {
            true => Ok(()),
            false => {
                let err = match status.code() {
                    Some(code) => Error::Exit { code },
                    None => Error::Terminated,
                };
                Err(echo::error(err))?
            }
        }
    }

    pub fn output(mut self) -> Result<Output> {
        let mut echo = self.echo();

        match self.inner.output() {
            Err(err) => {
                echo.end();
                Err(echo::error(err))?
            }
            Ok(output) => {
                echo.out("| read output".magenta());
                if !output.stdout.is_empty() {
                    echo.out(format!("stdout: {} bytes", output.stdout.len()));
                }
                if !output.stderr.is_empty() {
                    echo.out(format!("stderr: {} bytes", output.stderr.len()));
                }
                echo.end();
                Ok(output)
            }
        }
    }

    pub fn spawn(mut self) -> Result<Child> {
        self.echo().end();
        Ok(self.inner.spawn().map_err(echo::error)?)
    }

    pub fn pipe(mut self, mut to: Self) -> Result<Self> {
        self.inner.stdout(Stdio::piped());
        let mut child = self.spawn()?;
        let stdout = child.stdout.take().unwrap();
        to.inner.stdin(Stdio::from(stdout));
        to.piped = true;
        Ok(to)
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
