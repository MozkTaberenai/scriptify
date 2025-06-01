//! Simple command execution and piping functionality.

use crate::style::*;

use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, Command as StdCommand, Output, Stdio};
use std::thread;

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

/// Specifies which output streams should be piped between commands.
///
/// This enum controls how data flows between commands in a pipeline:
/// - **Between commands**: Determines which streams are connected
/// - **Final output**: The last command's stdout is captured unless specified otherwise
///
/// # Examples
///
/// ```no_run
/// use scriptify::{cmd, PipeMode};
///
/// // Pipe stdout (default)
/// let output = cmd!("echo", "hello")
///     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
///     .output()?;
///
/// // Pipe stderr between commands
/// let output = cmd!("command-with-errors")
///     .pipe(cmd!("grep", "ERROR"))
///     .pipe_stderr()
///     .output()?;
///
/// // Pipe both stdout and stderr
/// let output = cmd!("command-with-mixed-output")
///     .pipe(cmd!("sort"))
///     .pipe_both()
///     .output()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipeMode {
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
    commands: Vec<Cmd>,
    input: Option<String>,
    quiet: bool,
    pipe_mode: PipeMode,
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
            pipe_mode: PipeMode::Stdout,
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

    /// Set the pipe mode for this pipeline.
    ///
    /// This method allows you to specify which output streams should be connected
    /// between commands in the pipeline.
    ///
    /// # Arguments
    ///
    /// * `mode` - The pipe mode to use (Stdout, Stderr, or Both)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scriptify::{cmd, PipeMode};
    ///
    /// let output = cmd!("generate-data")
    ///     .pipe(cmd!("process-data"))
    ///     .pipe_mode(PipeMode::Stderr)
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_mode(mut self, mode: PipeMode) -> Self {
        self.pipe_mode = mode;
        self
    }

    /// Pipe stderr instead of stdout between commands.
    ///
    /// This is a convenience method equivalent to `pipe_mode(PipeMode::Stderr)`.
    /// Each command's stderr output becomes the next command's stdin input.
    ///
    /// # Use Cases
    ///
    /// - Error log processing: `error_producer.pipe(grep).pipe_stderr()`
    /// - Debugging pipelines: Separate error handling from normal data flow
    /// - Log analysis: Process error streams separately from output streams
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scriptify::cmd;
    ///
    /// // Process error messages through a pipeline
    /// let error_count = cmd!("sh", "-c", "echo 'ERROR: failed' >&2")
    ///     .pipe(cmd!("wc", "-l"))
    ///     .pipe_stderr()
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_stderr(self) -> Self {
        self.pipe_mode(PipeMode::Stderr)
    }

    /// Pipe both stdout and stderr combined between commands.
    ///
    /// This is a convenience method equivalent to `pipe_mode(PipeMode::Both)`.
    /// Both output streams are merged and sent to the next command's stdin.
    ///
    /// # Important Notes
    ///
    /// - **Order not guaranteed**: stdout and stderr are merged concurrently
    /// - **Performance**: May use additional threads for stream merging
    /// - **Buffering**: Some buffering may occur during the merge process
    ///
    /// # Use Cases
    ///
    /// - Log aggregation: Combine all output for unified processing
    /// - Debugging: Capture complete command output for analysis
    /// - Monitoring: Process all output streams together
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scriptify::cmd;
    ///
    /// // Sort all output (both stdout and stderr)
    /// let sorted_output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
    ///     .pipe(cmd!("sort"))
    ///     .pipe_both()
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_both(self) -> Self {
        self.pipe_mode(PipeMode::Both)
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
        // Supports all PipeMode variants:
        // - Stdout: Standard pipe between stdout->stdin
        // - Stderr: Pipe stderr->stdin using dedicated stderr pipes
        // - Both: Use try_clone() to merge stdout+stderr into single pipe
        //
        // This implementation leverages Rust 1.87.0's anonymous pipes with
        // try_clone() capability for efficient stream merging

        let mut children: Vec<Child> = Vec::new();
        let mut prev_reader: Option<std::io::PipeReader> = None;

        // Spawn all commands in the pipeline, connecting them with pipes
        for (i, cmd_def) in self.commands.iter().enumerate() {
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

            // Set up stdout and stderr based on pipe mode
            let is_last = i == self.commands.len() - 1;
            if is_last {
                // Last command: capture output if requested
                match self.pipe_mode {
                    PipeMode::Stdout => {
                        if capture_output {
                            cmd.stdout(Stdio::piped());
                        }
                    }
                    PipeMode::Stderr => {
                        if capture_output {
                            cmd.stdout(Stdio::piped());
                        }
                    }
                    PipeMode::Both => {
                        if capture_output {
                            // For combined mode, capture both streams to the same pipe
                            let (reader, writer) = std::io::pipe().map_err(|e| Error {
                                message: "Failed to create pipe for combined output".to_string(),
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
            } else {
                // Intermediate commands: create anonymous pipe for next command
                match self.pipe_mode {
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
                message: format!("Failed to spawn command: {}", cmd_def.program),
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
            match self.pipe_mode {
                PipeMode::Stdout => {
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
                PipeMode::Stderr => {
                    if let Some(last_child) = children.last_mut() {
                        if let Some(stdout) = last_child.stdout.take() {
                            let mut reader = BufReader::new(stdout);
                            reader.read_to_end(&mut result).map_err(|e| Error {
                                message: "Failed to read stdout from final command".to_string(),
                                source: Some(e),
                            })?;
                        }
                    }
                }
                PipeMode::Both => {
                    // For combined mode, read from the shared pipe reader
                    if let Some(reader) = prev_reader.take() {
                        let mut buf_reader = BufReader::new(reader);
                        buf_reader.read_to_end(&mut result).map_err(|e| Error {
                            message: "Failed to read combined output".to_string(),
                            source: Some(e),
                        })?;
                    }
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

        if let Some(cwd) = &cmd_def.cwd {
            cmd.current_dir(cwd);
        }

        cmd
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
mod cmd_tests {
    use super::*;

    #[test]
    fn test_cmd_new() {
        let cmd = Cmd::new("echo");
        assert_eq!(cmd.program, "echo");
        assert!(cmd.args.is_empty());
        assert!(!cmd.quiet);
    }

    #[test]
    fn test_cmd_with_args() {
        let cmd = cmd!("echo", "hello", "world");
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_cmd_builder() {
        let cmd = Cmd::new("ls")
            .arg("-la")
            .env("TEST", "value")
            .cwd("/tmp")
            .quiet();

        assert_eq!(cmd.program, "ls");
        assert_eq!(cmd.args, vec!["-la"]);
        assert_eq!(cmd.envs, vec![("TEST".to_string(), "value".to_string())]);
        assert_eq!(cmd.cwd, Some("/tmp".to_string()));
        assert!(cmd.quiet);
    }

    #[test]
    fn test_cmd_output() {
        let output = cmd!("echo", "test").quiet().output().unwrap();
        assert_eq!(output.trim(), "test");
    }

    #[test]
    fn test_cmd_with_input() {
        let output = cmd!("cat").input("hello world").quiet().output().unwrap();
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_pipeline() {
        let output = cmd!("echo", "hello")
            .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
            .quiet()
            .output()
            .unwrap();
        assert_eq!(output.trim(), "HELLO");
    }

    #[test]
    fn test_pipeline_with_input() {
        let output = cmd!("tr", "[:lower:]", "[:upper:]")
            .input("hello world")
            .quiet()
            .output()
            .unwrap();
        assert_eq!(output.trim(), "HELLO WORLD");
    }

    #[test]
    fn test_environment_variable() {
        // Test that environment variables are properly set for the process
        let output = cmd!("printenv", "TEST_VAR")
            .env("TEST_VAR", "test_value")
            .quiet()
            .output()
            .unwrap();
        assert_eq!(output.trim(), "test_value");
    }

    #[test]
    fn test_error_handling() {
        let result = cmd!("nonexistent_command_12345").quiet().run();
        assert!(result.is_err());
    }

    #[test]
    fn test_quiet_mode() {
        // This test mainly checks that quiet mode doesn't crash
        let result = cmd!("echo", "test").quiet().run();
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_pipes() {
        let output = cmd!("echo", "hello world")
            .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
            .pipe(cmd!("rev"))
            .quiet()
            .output()
            .unwrap();
        assert_eq!(output.trim(), "DLROW OLLEH");
    }

    #[test]
    fn test_pipe_stderr() {
        // Test piping stderr to next command
        // First command generates stderr, second command should receive it
        let output = cmd!("sh", "-c", "echo 'error message' >&2")
            .pipe(cmd!("wc", "-c"))
            .pipe_stderr()
            .quiet()
            .output()
            .unwrap();

        // Should count characters in the stderr message (13 chars + newline = 14)
        assert_eq!(output.trim(), "14");
    }

    #[test]
    fn test_pipe_both() {
        // Test piping both stdout and stderr
        let output = cmd!("sh", "-c", "echo 'stdout' && echo 'stderr' >&2")
            .pipe(cmd!("sort"))
            .pipe_both()
            .quiet()
            .output()
            .unwrap();

        // Should contain both outputs (order may vary due to threading)
        let output_str = output.trim();
        assert!(output_str.contains("stdout"));
        assert!(output_str.contains("stderr"));
    }

    #[test]
    fn test_pipe_mode_explicit() {
        // Test setting pipe mode explicitly
        let output = cmd!("echo", "test")
            .pipe(cmd!("cat"))
            .pipe_mode(PipeMode::Stdout)
            .quiet()
            .output()
            .unwrap();

        assert_eq!(output.trim(), "test");
    }

    #[test]
    fn test_default_pipe_mode() {
        // Test that default pipe mode is Stdout
        let pipeline = cmd!("echo", "test").pipe(cmd!("cat"));
        assert_eq!(pipeline.pipe_mode, PipeMode::Stdout);
    }

    #[test]
    fn test_native_pipeline_for_all_modes() {
        // Test that all pipe modes work with native pipeline implementation
        // This test ensures try_native_pipeline supports all modes instead of falling back

        // Test stdout mode (should use native pipeline)
        let stdout_result = cmd!("echo", "native test")
            .pipe(cmd!("cat"))
            .pipe_mode(PipeMode::Stdout)
            .quiet()
            .output()
            .unwrap();
        assert_eq!(stdout_result.trim(), "native test");

        // Test stderr mode (uses native pipeline with try_clone)
        let stderr_result = cmd!("sh", "-c", "echo 'native error' >&2")
            .pipe(cmd!("wc", "-c"))
            .pipe_mode(PipeMode::Stderr)
            .quiet()
            .output()
            .unwrap();
        assert_eq!(stderr_result.trim(), "13");

        // Test both mode (uses native pipeline with try_clone)
        let both_result = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
            .pipe(cmd!("wc", "-l"))
            .pipe_mode(PipeMode::Both)
            .quiet()
            .output()
            .unwrap();
        assert_eq!(both_result.trim(), "2");
    }
}
