//! Command implementation and execution logic.

use crate::cmd::{error::Error, types::*};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::Path;

impl Cmd {
    /// Create a new command.
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_os_string(),
            args: Vec::new(),
            envs: Vec::new(),
            current_dir: None,
            suppress_echo: false,
        }
    }

    /// Add an argument.
    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    /// Add multiple arguments.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_os_string());
        }
        self
    }

    /// Set an environment variable.
    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), val.as_ref().to_os_string()));
        self
    }

    /// Set the working directory.
    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.current_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Convert this command into a single-command pipeline.
    pub(crate) fn into_pipeline(self) -> Pipeline {
        let suppress_echo = self.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout)],
            input: None,
            suppress_echo,
        }
    }

    /// Set binary input data for the command.
    /// Accepts Vec<u8>, &[u8], or other types that can be converted to Vec<u8>.
    pub fn input_bytes(self, input: impl AsRef<[u8]>) -> Pipeline {
        self.into_pipeline().input_bytes(input)
    }

    /// Set binary input data for the command with zero-copy optimization.
    /// Takes ownership of Vec<u8> to avoid copying.
    pub fn input_bytes_owned(self, bytes: Vec<u8>) -> Pipeline {
        self.into_pipeline().input_bytes_owned(bytes)
    }

    /// Set text input for the command.
    /// Optimized to convert string directly to bytes without intermediate allocation.
    pub fn input(self, input: impl AsRef<str>) -> Pipeline {
        self.into_pipeline().input(input)
    }

    /// Set input from a Reader (unbuffered).
    /// Reads all data from the reader into memory.
    /// For large files, consider using `input_buffered` instead.
    pub fn input_reader<R: Read + Send + 'static>(self, reader: R) -> Pipeline {
        self.into_pipeline().input_reader(reader)
    }

    /// Set input from a Reader with automatic buffering.
    /// More efficient for large files or slow readers.
    pub fn input_buffered<R: Read + Send + 'static>(self, reader: R) -> Pipeline {
        self.into_pipeline().input_buffered(reader)
    }

    /// Run without echoing the command.
    pub fn no_echo(mut self) -> Self {
        self.suppress_echo = true;
        self
    }

    /// Pipe this command to another command.
    pub fn pipe(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stdout)],
            input: None,
            suppress_echo,
        }
    }

    /// Pipe this command's stderr to another command's stdin.
    ///
    /// This is a convenience method that creates a pipeline with PipeMode::Stderr,
    /// allowing you to specify the pipe mode directly in the builder pattern.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scriptify::cmd;
    ///
    /// // Process error messages through a pipeline
    /// let error_count = cmd!("sh", "-c", "echo 'ERROR: failed' >&2")
    ///     .pipe_stderr(cmd!("wc", "-l"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_stderr(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stderr)],
            input: None,
            suppress_echo,
        }
    }

    /// Pipe this command's combined stdout and stderr to another command's stdin.
    ///
    /// This is a convenience method that creates a pipeline with PipeMode::Both,
    /// allowing you to specify the pipe mode directly in the builder pattern.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scriptify::cmd;
    ///
    /// // Sort all output (both stdout and stderr)
    /// let sorted_output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
    ///     .pipe_both(cmd!("sort"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_both(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Both)],
            input: None,
            suppress_echo,
        }
    }

    /// Run the command and return the exit status.
    pub fn run(self) -> Result<(), Error> {
        self.into_pipeline().run()
    }

    /// Get binary output from the command.
    pub fn output_bytes(self) -> Result<Vec<u8>, Error> {
        self.into_pipeline().output_bytes()
    }

    /// Get text output from the command.
    pub fn output(self) -> Result<String, Error> {
        self.into_pipeline().output()
    }

    /// Run the command and stream output to a Writer.
    /// This is more memory-efficient for large outputs.
    pub fn stream_to<W: Write>(self, writer: W) -> Result<(), Error> {
        self.into_pipeline().stream_to(writer)
    }

    /// Run the command with both input Reader and output Writer.
    /// This is the most flexible method for streaming I/O.
    pub fn run_with_io<R: Read + Send + 'static, W: Write>(
        self,
        reader: R,
        writer: W,
    ) -> Result<(), Error> {
        self.into_pipeline().run_with_io(reader, writer)
    }

    /// Spawn the command with I/O control.
    pub fn spawn_with_io(self) -> Result<PipelineSpawn, Error> {
        self.into_pipeline().spawn_with_io()
    }

    /// Spawn the command with stdin control.
    pub fn spawn_with_stdin(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStdin>), Error> {
        self.into_pipeline().spawn_with_stdin()
    }

    /// Spawn the command with stdout control.
    pub fn spawn_with_stdout(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStdout>), Error> {
        self.into_pipeline().spawn_with_stdout()
    }

    /// Spawn the command with stderr control.
    pub fn spawn_with_stderr(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStderr>), Error> {
        self.into_pipeline().spawn_with_stderr()
    }

    /// Spawn the command with both stdout and stderr control.
    pub fn spawn_with_both(
        self,
    ) -> Result<
        (
            PipelineHandle,
            Option<std::process::ChildStdout>,
            Option<std::process::ChildStderr>,
        ),
        Error,
    > {
        self.into_pipeline().spawn_with_both()
    }

    /// Quotes an argument for display if it contains characters that affect readability.  
    ///
    /// This function focuses on readability rather than shell compatibility:
    /// - Arguments with spaces or control characters: wrapped in single quotes with escaping
    /// - Arguments with single quotes: wrapped in double quotes with escaping
    /// - Empty arguments: displayed as empty quotes
    /// - Safe arguments: displayed as-is
    pub(crate) fn quote_argument(arg: &OsStr) -> String {
        let arg_str = arg.to_string_lossy();

        // If the argument is empty, return empty quotes
        if arg_str.is_empty() {
            return "\"\"".to_string();
        }

        // Check if argument needs quoting (focus on readability and security)
        let needs_quoting = arg_str.chars().any(|c| {
            matches!(
                c,
                ' ' | '\t' | '\n' | '\r' | '"' | '\'' | '\0'
                    ..='\x1F'
                        | '\x7F'
                        | '*'
                        | '?'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '~'
                        | '$'
                        | '`'
                        | '|'
                        | '&'
                        | ';'
                        | '('
                        | ')'
                        | '<'
                        | '>'
                        | '#'
                        | '!'
                        | '='
            )
        });

        // Escape control characters for better display
        let escape_control_chars = |s: &str| -> String {
            s.chars()
                .map(|c| match c {
                    '\t' => "\\t".to_string(),
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\0' => "\\0".to_string(),
                    c if c.is_control() => format!("\\x{:02x}", c as u8),
                    c => c.to_string(),
                })
                .collect()
        };

        // Handle arguments with single quotes specially
        if arg_str.contains('\'') {
            let escaped = arg_str.replace('\\', "\\\\").replace('"', "\\\"");
            let escaped = escape_control_chars(&escaped);
            return format!("\"{}\"", escaped);
        }

        // If argument needs quoting, use single quotes with control char escaping
        if needs_quoting {
            let escaped = escape_control_chars(&arg_str);
            return format!("'{}'", escaped);
        }

        // No quoting needed
        arg_str.to_string()
    }
}
