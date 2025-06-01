//! Simple command execution and piping functionality.

use crate::style::*;

use std::path::Path;
use std::process::{Command as StdCommand, Output, Stdio};

/// A simple command builder.
#[derive(Debug, Clone)]
pub struct Cmd {
    program: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    cwd: Option<String>,
    input: Option<String>,
    quiet: bool,
}

/// A pipeline of commands.
#[derive(Debug)]
pub struct Pipeline {
    commands: Vec<Cmd>,
    input: Option<String>,
    quiet: bool,
}

/// Command execution error.
#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<std::io::Error>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e as &dyn std::error::Error)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            message: "Command execution failed".to_string(),
            source: Some(err),
        }
    }
}

impl Cmd {
    /// Create a new command.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            envs: Vec::new(),
            cwd: None,
            input: None,
            quiet: false,
        }
    }

    /// Parse a command string (basic implementation).
    pub fn parse(cmd: &str) -> Self {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Self::new("");
        }

        let mut command = Self::new(parts[0]);
        for arg in &parts[1..] {
            command = command.arg(*arg);
        }
        command
    }

    /// Add an argument.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple arguments.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    /// Set an environment variable.
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.envs.push((key.into(), val.into()));
        self
    }

    /// Set the working directory.
    pub fn cwd(mut self, dir: impl AsRef<Path>) -> Self {
        self.cwd = Some(dir.as_ref().to_string_lossy().to_string());
        self
    }

    /// Set input for the command.
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(input.into());
        self
    }

    /// Run quietly without echoing.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Pipe this command to another command.
    pub fn pipe(self, next: Cmd) -> Pipeline {
        let quiet = self.quiet;
        Pipeline {
            commands: vec![self, next],
            input: None,
            quiet,
        }
    }

    /// Run the command and return the exit status.
    pub fn run(self) -> Result<(), Error> {
        self.execute_internal(false, false).map(|_| ())
    }

    /// Run the command and return the output as a string.
    pub fn output(self) -> Result<String, Error> {
        let output = self.execute_internal(true, false)?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn execute_internal(self, capture_output: bool, pipe_stdin: bool) -> Result<Output, Error> {
        if !self.quiet {
            self.echo_command();
        }

        let mut cmd = StdCommand::new(&self.program);
        cmd.args(&self.args);

        for (key, val) in &self.envs {
            cmd.env(key, val);
        }

        if let Some(cwd) = &self.cwd {
            cmd.current_dir(cwd);
        }

        if capture_output {
            cmd.stdout(Stdio::piped());
        }

        if pipe_stdin || self.input.is_some() {
            cmd.stdin(Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(|e| Error {
            message: format!("Failed to spawn command: {}", self.program),
            source: Some(e),
        })?;

        if let Some(input) = &self.input {
            if let Some(stdin) = child.stdin.take() {
                use std::io::Write;
                let mut stdin = stdin;
                stdin.write_all(input.as_bytes()).map_err(|e| Error {
                    message: "Failed to write to stdin".to_string(),
                    source: Some(e),
                })?;
                drop(stdin); // Close stdin to signal EOF
            }
        }

        child.wait_with_output().map_err(|e| Error {
            message: "Failed to wait for command".to_string(),
            source: Some(e),
        })
    }

    fn echo_command(&self) {
        let mut echo = crate::Echo::new();
        echo = echo.sput("cmd", BRIGHT_BLACK);

        if let Some(cwd) = &self.cwd {
            echo = echo
                .sput("cd:", BRIGHT_BLUE)
                .sput(cwd, UNDERLINE_BRIGHT_BLUE);
        }

        for (key, val) in &self.envs {
            echo = echo
                .sput("env:", BRIGHT_BLUE)
                .sput(key, UNDERLINE_BRIGHT_BLUE)
                .put("=")
                .sput(val, UNDERLINE_BRIGHT_BLUE);
        }

        echo = echo.sput(&self.program, BOLD_CYAN);

        for arg in &self.args {
            echo = echo.sput(arg, BOLD_UNDERLINE);
        }

        echo.end();
    }
}

impl Pipeline {
    /// Add another command to the pipeline.
    pub fn pipe(mut self, cmd: Cmd) -> Self {
        self.commands.push(cmd);
        self
    }

    /// Set input for the entire pipeline.
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(input.into());
        self
    }

    /// Run quietly without echoing.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Run the pipeline.
    pub fn run(self) -> Result<(), Error> {
        self.execute_internal(false).map(|_| ())
    }

    /// Run the pipeline and return the output as a string.
    pub fn output(self) -> Result<String, Error> {
        let output = self.execute_internal(true)?;
        Ok(String::from_utf8_lossy(&output).to_string())
    }

    fn execute_internal(self, capture_output: bool) -> Result<Vec<u8>, Error> {
        if !self.quiet {
            self.echo_pipeline();
        }

        if self.commands.is_empty() {
            return Ok(Vec::new());
        }

        // For single command, use the simpler Cmd execution
        if self.commands.len() == 1 {
            let mut cmd = self.commands.into_iter().next().unwrap();
            if let Some(input) = self.input {
                cmd = cmd.input(input);
            }
            cmd = cmd.quiet();
            return if capture_output {
                Ok(cmd.output()?.into_bytes())
            } else {
                cmd.run()?;
                Ok(Vec::new())
            };
        }

        // For multiple commands, create a proper pipeline using shell
        let mut shell_command = String::new();
        for (i, cmd) in self.commands.iter().enumerate() {
            if i > 0 {
                shell_command.push_str(" | ");
            }
            shell_command.push_str(&cmd.program);
            for arg in &cmd.args {
                shell_command.push(' ');
                shell_command.push_str(arg);
            }
        }

        let mut final_cmd = if cfg!(target_os = "windows") {
            Cmd::new("cmd").arg("/C").arg(&shell_command)
        } else {
            Cmd::new("sh").arg("-c").arg(&shell_command)
        }
        .quiet();

        // Apply environment variables from first command
        if let Some(first_cmd) = self.commands.first() {
            for (key, val) in &first_cmd.envs {
                final_cmd = final_cmd.env(key, val);
            }
            if let Some(cwd) = &first_cmd.cwd {
                final_cmd = final_cmd.cwd(cwd);
            }
        }

        if let Some(input) = self.input {
            final_cmd = final_cmd.input(input);
        }

        if capture_output {
            Ok(final_cmd.output()?.into_bytes())
        } else {
            final_cmd.run()?;
            Ok(Vec::new())
        }
    }

    fn echo_pipeline(&self) {
        let mut echo = crate::Echo::new();
        echo = echo.sput("cmd", BRIGHT_BLACK);

        for (i, cmd) in self.commands.iter().enumerate() {
            if i > 0 {
                echo = echo.sput("|", MAGENTA);
            }

            if let Some(cwd) = &cmd.cwd {
                echo = echo
                    .sput("cd:", BRIGHT_BLUE)
                    .sput(cwd, UNDERLINE_BRIGHT_BLUE);
            }

            for (key, val) in &cmd.envs {
                echo = echo
                    .sput("env:", BRIGHT_BLUE)
                    .sput(key, UNDERLINE_BRIGHT_BLUE)
                    .put("=")
                    .sput(val, UNDERLINE_BRIGHT_BLUE);
            }

            echo = echo.sput(&cmd.program, BOLD_CYAN);

            for arg in &cmd.args {
                echo = echo.sput(arg, BOLD_UNDERLINE);
            }
        }

        echo.end();
    }
}

/// Macro to create a new command.
#[macro_export]
macro_rules! cmd {
    ($program:expr) => {
        $crate::Cmd::new($program)
    };
    ($program:expr, $($arg:expr),* $(,)?) => {
        $crate::Cmd::new($program)$(.arg($arg))*
    };
}

#[cfg(test)]
mod tests;
