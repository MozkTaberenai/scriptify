//! Simple command execution and piping functionality.

use crate::style::*;

use std::ffi::{OsStr, OsString};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command as StdCommand, Output, Stdio};
use std::thread;

/// A simple command builder.
#[derive(Debug, Clone)]
pub struct Cmd {
    program: OsString,
    args: Vec<OsString>,
    envs: Vec<(OsString, OsString)>,
    current_dir: Option<PathBuf>,
    input: Option<String>,
    quiet: bool,
}

/// Specifies which output streams should be piped between commands.
///
/// This enum is used internally to track pipe modes, but you typically don't need
/// to use it directly. Instead, use the convenient builder methods on `Cmd`:
///
/// - `pipe(cmd)` - pipes stdout (default)
/// - `pipe_stderr(cmd)` - pipes stderr only
/// - `pipe_both(cmd)` - pipes both stdout and stderr combined
///
/// # Examples
///
/// ```no_run
/// use scriptify::cmd;
///
/// // Pipe stdout (default)
/// let output = cmd!("echo", "hello")
///     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
///     .output()?;
///
/// // Pipe stderr between commands
/// let output = cmd!("command-with-errors")
///     .pipe_stderr(cmd!("grep", "ERROR"))
///     .output()?;
///
/// // Pipe both stdout and stderr
/// let output = cmd!("command-with-mixed-output")
///     .pipe_both(cmd!("sort"))
///     .output()?;
///
/// // Mixed pipe modes in one pipeline
/// let output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
///     .pipe_stderr(cmd!("process-errors"))
///     .pipe(cmd!("process-output"))
///     .output()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeMode {
    /// Pipe only stdout between commands (default behavior).
    ///
    /// This is the standard Unix pipe behavior where each command's stdout
    /// becomes the next command's stdin.
    Stdout,

    /// Pipe only stderr between commands.
    ///
    /// Each command's stderr becomes the next command's stdin, while stdout
    /// is not connected between commands. Useful for error processing pipelines.
    Stderr,

    /// Pipe both stdout and stderr combined between commands.
    ///
    /// Both output streams are merged and sent to the next command's stdin.
    /// Note: The order of merged output may vary due to concurrent execution.
    Both,
}

/// A pipeline of commands.
#[derive(Debug)]
pub struct Pipeline {
    connections: Vec<(Cmd, PipeMode)>,
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
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_os_string(),
            args: Vec::new(),
            envs: Vec::new(),
            current_dir: None,
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
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stdout)],
            input: None,
            quiet,
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
        let quiet = self.quiet;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stderr)],
            input: None,
            quiet,
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
        let quiet = self.quiet;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Both)],
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

        if let Some(current_dir) = &self.current_dir {
            cmd.current_dir(current_dir);
        }

        if capture_output {
            cmd.stdout(Stdio::piped());
        }

        if pipe_stdin || self.input.is_some() {
            cmd.stdin(Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(|e| Error {
            message: format!(
                "Failed to spawn command: {}",
                self.program.to_string_lossy()
            ),
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

        let output = child.wait_with_output().map_err(|e| Error {
            message: "Failed to wait for command".to_string(),
            source: Some(e),
        })?;

        if !output.status.success() {
            return Err(Error {
                message: format!("Command failed with exit code: {:?}", output.status.code()),
                source: None,
            });
        }

        Ok(output)
    }

    /// Quotes an argument for display if it contains characters that affect readability.
    ///
    /// This function focuses on readability rather than shell compatibility:
    /// - Arguments with spaces or control characters: wrapped in single quotes with escaping
    /// - Arguments with single quotes: wrapped in double quotes with escaping
    /// - Empty arguments: displayed as empty quotes
    /// - Safe arguments: displayed as-is
    fn quote_argument(arg: &OsStr) -> String {
        let arg_str = arg.to_string_lossy();

        // If the argument is empty, return empty quotes
        if arg_str.is_empty() {
            return "\"\"".to_string();
        }

        // Check if argument needs quoting (focus on readability)
        let needs_quoting = arg_str.chars().any(|c| {
            matches!(
                c,
                ' ' | '\t' | '\n' | '\r' | '"' | '\'' | '\0'..='\x1F' | '\x7F'
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

    fn echo_command(&self) {
        let mut echo = crate::Echo::new();
        echo = echo.sput("cmd", BRIGHT_BLACK);

        if let Some(current_dir) = &self.current_dir {
            let quoted_dir = Self::quote_argument(current_dir.as_os_str());
            echo = echo
                .sput("cd:", BRIGHT_BLUE)
                .sput(quoted_dir, UNDERLINE_BRIGHT_BLUE);
        }

        for (key, val) in &self.envs {
            let quoted_key = Self::quote_argument(key);
            let quoted_val = Self::quote_argument(val);
            echo = echo
                .sput("env:", BRIGHT_BLUE)
                .sput(quoted_key, UNDERLINE_BRIGHT_BLUE)
                .put("=")
                .sput(quoted_val, UNDERLINE_BRIGHT_BLUE);
        }

        let quoted_program = Self::quote_argument(&self.program);
        echo = echo.sput(quoted_program, BOLD_CYAN);

        for arg in &self.args {
            let quoted_arg = Self::quote_argument(arg);
            echo = echo.sput(quoted_arg, BOLD_UNDERLINE);
        }

        echo.end();
    }
}

impl Pipeline {
    /// Add another command to the pipeline.
    pub fn pipe(mut self, cmd: Cmd) -> Self {
        self.connections.push((cmd, PipeMode::Stdout));
        self
    }

    /// Add another command to the pipeline, piping stderr.
    pub fn pipe_stderr(mut self, cmd: Cmd) -> Self {
        self.connections.push((cmd, PipeMode::Stderr));
        self
    }

    /// Add another command to the pipeline, piping both stdout and stderr.
    pub fn pipe_both(mut self, cmd: Cmd) -> Self {
        self.connections.push((cmd, PipeMode::Both));
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

        if self.connections.is_empty() {
            return Ok(Vec::new());
        }

        // For single command, use the simpler Cmd execution
        if self.connections.len() == 1 {
            let mut cmd = self.connections.into_iter().next().unwrap().0;
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

        // Use native pipeline with std::io::pipe for efficiency
        self.execute_native_pipeline(capture_output)
    }

    fn execute_native_pipeline(self, capture_output: bool) -> Result<Vec<u8>, Error> {
        // Native pipeline implementation using std::io::pipe (Rust 1.87.0+)
        // This provides several advantages:
        // 1. Memory efficiency: streaming data instead of buffering everything
        // 2. Better error handling: individual process status tracking
        // 3. Platform independence: no shell dependency
        // 4. Performance: reduced context switching and process overhead
        // 5. All PipeMode variants supported natively with try_clone()
        self.try_native_pipeline(capture_output)
    }

    fn try_native_pipeline(&self, capture_output: bool) -> Result<Vec<u8>, Error> {
        // Native pipeline implementation using std::io::pipe from Rust 1.87.0
        // This creates anonymous pipes directly in memory, enabling true streaming processing
        //
        // Now supports individual PipeMode per connection:
        // - Each connection between commands can have its own pipe mode
        // - connections[i].1 specifies how connections[i-1] pipes to connections[i]

        let mut children: Vec<Child> = Vec::new();
        let mut prev_reader: Option<std::io::PipeReader> = None;

        // Spawn all commands in the pipeline, connecting them with pipes
        for (i, (cmd_def, _pipe_mode)) in self.connections.iter().enumerate() {
            let mut cmd = Self::build_std_command_static(cmd_def);

            // Set up stdin
            if i == 0 {
                // First command: use input if provided
                if self.input.is_some() {
                    cmd.stdin(Stdio::piped());
                }
            } else {
                // Subsequent commands: use previous command's output
                if let Some(reader) = prev_reader.take() {
                    cmd.stdin(Stdio::from(reader));
                }
            }

            // Set up stdout and stderr based on this connection's pipe mode
            let is_last = i == self.connections.len() - 1;
            if is_last {
                // Last command: capture output if requested
                if capture_output {
                    cmd.stdout(Stdio::piped());
                }
            } else {
                // Get the next command's pipe mode to determine how to pipe
                let next_pipe_mode = self.connections[i + 1].1;
                match next_pipe_mode {
                    PipeMode::Stdout => {
                        let (reader, writer) = std::io::pipe().map_err(|e| Error {
                            message: "Failed to create stdout pipe".to_string(),
                            source: Some(e),
                        })?;
                        cmd.stdout(Stdio::from(writer));
                        prev_reader = Some(reader);
                    }
                    PipeMode::Stderr => {
                        let (reader, writer) = std::io::pipe().map_err(|e| Error {
                            message: "Failed to create stderr pipe".to_string(),
                            source: Some(e),
                        })?;
                        cmd.stderr(Stdio::from(writer));
                        prev_reader = Some(reader);
                    }
                    PipeMode::Both => {
                        // For combined mode, both stdout and stderr write to the same pipe
                        let (reader, writer) = std::io::pipe().map_err(|e| Error {
                            message: "Failed to create combined pipe".to_string(),
                            source: Some(e),
                        })?;
                        let writer_clone = writer.try_clone().map_err(|e| Error {
                            message: "Failed to clone pipe writer".to_string(),
                            source: Some(e),
                        })?;
                        cmd.stdout(Stdio::from(writer));
                        cmd.stderr(Stdio::from(writer_clone));
                        prev_reader = Some(reader);
                    }
                }
            }

            let mut child = cmd.spawn().map_err(|e| Error {
                message: format!(
                    "Failed to spawn command: {}",
                    cmd_def.program.to_string_lossy()
                ),
                source: Some(e),
            })?;

            // Handle input for the first command
            // Use a separate thread to avoid blocking the main execution
            if i == 0 {
                if let Some(input) = &self.input {
                    if let Some(stdin) = child.stdin.take() {
                        let input_clone = input.clone();
                        thread::spawn(move || {
                            let mut stdin = stdin;
                            let _ = stdin.write_all(input_clone.as_bytes());
                            // stdin is automatically closed when it goes out of scope
                        });
                    }
                }
            }

            children.push(child);
        }

        // Collect output from the last command if needed
        let mut result = Vec::new();
        if capture_output {
            if let Some(last_child) = children.last_mut() {
                if let Some(stdout) = last_child.stdout.take() {
                    let mut reader = BufReader::new(stdout);
                    reader.read_to_end(&mut result).map_err(|e| Error {
                        message: "Failed to read stdout".to_string(),
                        source: Some(e),
                    })?;
                }
            }
        }

        // Wait for all children to complete
        for mut child in children {
            let status = child.wait().map_err(|e| Error {
                message: "Failed to wait for command".to_string(),
                source: Some(e),
            })?;

            if !status.success() {
                return Err(Error {
                    message: format!("Command failed with exit code: {:?}", status.code()),
                    source: None,
                });
            }
        }

        Ok(result)
    }

    fn build_std_command_static(cmd_def: &Cmd) -> StdCommand {
        let mut cmd = StdCommand::new(&cmd_def.program);
        cmd.args(&cmd_def.args);

        for (key, val) in &cmd_def.envs {
            cmd.env(key, val);
        }

        if let Some(current_dir) = &cmd_def.current_dir {
            cmd.current_dir(current_dir);
        }

        cmd
    }

    fn echo_pipeline(&self) {
        let mut echo = crate::Echo::new();
        echo = echo.sput("cmd", BRIGHT_BLACK);

        for (i, (cmd, pipe_mode)) in self.connections.iter().enumerate() {
            if i > 0 {
                let pipe_symbol = match pipe_mode {
                    PipeMode::Stdout => "|",
                    PipeMode::Stderr => "|&",
                    PipeMode::Both => "|&&",
                };
                echo = echo.sput(pipe_symbol, MAGENTA);
            }

            if let Some(current_dir) = &cmd.current_dir {
                let quoted_dir = Cmd::quote_argument(current_dir.as_os_str());
                echo = echo
                    .sput("cd:", BRIGHT_BLUE)
                    .sput(quoted_dir, UNDERLINE_BRIGHT_BLUE);
            }

            for (key, val) in &cmd.envs {
                let quoted_key = Cmd::quote_argument(key);
                let quoted_val = Cmd::quote_argument(val);
                echo = echo
                    .sput("env:", BRIGHT_BLUE)
                    .sput(quoted_key, UNDERLINE_BRIGHT_BLUE)
                    .put("=")
                    .sput(quoted_val, UNDERLINE_BRIGHT_BLUE);
            }

            let quoted_program = Cmd::quote_argument(&cmd.program);
            echo = echo.sput(quoted_program, BOLD_CYAN);

            for arg in &cmd.args {
                let quoted_arg = Cmd::quote_argument(arg);
                echo = echo.sput(quoted_arg, BOLD_UNDERLINE);
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
#[path = "tests.rs"]
mod tests;
