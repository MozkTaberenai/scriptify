//! Pipeline implementation and execution logic.

use crate::cmd::{error::Error, types::*};
use crate::style::*;
use std::io::{BufReader, Read, Write};
use std::process::{Child, Command as StdCommand, Stdio};
use std::thread;

impl PipelineHandle {
    /// Wait for all processes in the pipeline to complete.
    pub fn wait(self) -> Result<(), Error> {
        for mut child in self.children {
            let status = child.wait().map_err(|e| Error {
                message: "Failed to wait for child process".to_string(),
                source: Some(e),
            })?;

            if !status.success() {
                return Err(Error {
                    message: format!("Command failed with exit code: {:?}", status.code()),
                    source: None,
                });
            }
        }
        Ok(())
    }

    /// Collect output from the last command in the pipeline.
    /// Note: This only works if the pipeline was spawned with stdout captured.
    pub fn output(self) -> Result<String, Error> {
        let bytes = self.output_bytes()?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Collect binary output from the last command in the pipeline.
    /// Note: This only works if the pipeline was spawned with stdout captured.
    pub fn output_bytes(mut self) -> Result<Vec<u8>, Error> {
        if let Some(last_child) = self.children.last_mut() {
            if let Some(stdout) = last_child.stdout.take() {
                use std::io::Read;
                let mut output = Vec::new();
                let mut reader = BufReader::new(stdout);
                reader.read_to_end(&mut output).map_err(|e| Error {
                    message: "Failed to read stdout".to_string(),
                    source: Some(e),
                })?;

                // Wait for the process to complete
                for mut child in self.children {
                    child.wait().map_err(|e| Error {
                        message: "Failed to wait for child process".to_string(),
                        source: Some(e),
                    })?;
                }

                return Ok(output);
            }
        }

        Err(Error {
            message: "No stdout available to read from".to_string(),
            source: None,
        })
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

    /// Set binary input data for the pipeline.
    /// Accepts Vec<u8>, &[u8], or other types that can be converted to Vec<u8>.
    pub fn input_bytes(mut self, input: impl AsRef<[u8]>) -> Self {
        self.input = Some(CmdInput::Bytes(input.as_ref().to_vec()));
        self
    }

    /// Set binary input data for the pipeline with zero-copy optimization.
    /// Takes ownership of Vec<u8> to avoid copying.
    pub fn input_bytes_owned(mut self, bytes: Vec<u8>) -> Self {
        self.input = Some(CmdInput::Bytes(bytes));
        self
    }

    /// Set text input for the pipeline (deprecated: use spawn_with_io for more control).
    /// This is kept for backward compatibility but users should prefer the spawn_with_* methods.
    pub fn input(mut self, input: impl AsRef<str>) -> Self {
        self.input = Some(CmdInput::Bytes(input.as_ref().as_bytes().to_vec()));
        self
    }

    /// Set input from a Reader (deprecated: use spawn_with_stdin for more control).
    /// Note: This will cause an error with output() - use spawn_with_* methods instead.
    pub fn input_reader<R: Read + Send + 'static>(mut self, reader: R) -> Self {
        self.input = Some(CmdInput::Reader(Box::new(reader)));
        self
    }

    /// Set input from a Reader with automatic buffering (deprecated: use spawn_with_stdin for more control).
    /// Note: This will cause an error with output() - use spawn_with_* methods instead.
    pub fn input_buffered<R: Read + Send + 'static>(self, reader: R) -> Self {
        self.input_reader(BufReader::new(reader))
    }

    /// Run without echoing the pipeline.
    pub fn no_echo(mut self) -> Self {
        self.suppress_echo = true;
        self
    }

    /// Run the pipeline.
    pub fn run(self) -> Result<(), Error> {
        self.execute_internal(false).map(|_| ())
    }

    /// Run the pipeline and return the output as a string.
    /// Get binary output from the pipeline.
    pub fn output_bytes(self) -> Result<Vec<u8>, Error> {
        self.execute_internal(true)
    }

    /// Get text output from the pipeline.
    pub fn output(self) -> Result<String, Error> {
        let bytes = self.output_bytes()?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Spawn pipeline with full I/O access.
    /// User is responsible for managing stdin, stdout, and stderr in separate threads.
    pub fn spawn_with_io(self) -> Result<PipelineSpawn, Error> {
        if !self.suppress_echo {
            self.echo_pipeline();
        }

        if self.connections.is_empty() {
            return Ok(PipelineSpawn {
                handle: PipelineHandle {
                    children: Vec::new(),
                },
                stdin: None,
                stdout: None,
                stderr: None,
            });
        }

        // For single command, handle it specially
        if self.connections.len() == 1 {
            let cmd = self.connections.into_iter().next().unwrap().0;
            let mut std_cmd = Self::build_std_command_static(&cmd);

            // Set up I/O - always enable stdin for compatibility
            std_cmd.stdin(Stdio::piped());
            std_cmd.stdout(Stdio::piped());
            std_cmd.stderr(Stdio::piped());

            let mut child = std_cmd.spawn().map_err(|e| Error {
                message: format!("Failed to spawn command: {}", cmd.program.to_string_lossy()),
                source: Some(e),
            })?;

            let stdin = child.stdin.take();
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            // For single commands, input handling is done by existing methods for backward compatibility

            return Ok(PipelineSpawn {
                handle: PipelineHandle {
                    children: vec![child],
                },
                stdin,
                stdout,
                stderr,
            });
        }

        // Multi-command pipeline
        let mut children: Vec<Child> = Vec::new();
        let mut prev_reader: Option<std::io::PipeReader> = None;
        let mut first_stdin = None;
        let mut last_stdout = None;
        let mut last_stderr = None;

        // Spawn all commands in the pipeline
        for (i, (cmd_def, _pipe_mode)) in self.connections.iter().enumerate() {
            let mut cmd = Self::build_std_command_static(cmd_def);

            // Set up stdin
            if i == 0 {
                // First command: set up for potential input
                cmd.stdin(Stdio::piped());
            } else {
                // Subsequent commands: use previous command's output
                if let Some(reader) = prev_reader.take() {
                    cmd.stdin(Stdio::from(reader));
                }
            }

            // Set up stdout and stderr
            let is_last = i == self.connections.len() - 1;
            if is_last {
                // Last command: capture both stdout and stderr
                cmd.stdout(Stdio::piped());
                cmd.stderr(Stdio::piped());
            } else {
                // Intermediate commands: pipe to next command
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

            // Capture I/O handles
            if i == 0 {
                first_stdin = child.stdin.take();
            }
            if is_last {
                last_stdout = child.stdout.take();
                last_stderr = child.stderr.take();
            }

            children.push(child);
        }

        // For pipelines, input handling is now user's responsibility via spawn API

        Ok(PipelineSpawn {
            handle: PipelineHandle { children },
            stdin: first_stdin,
            stdout: last_stdout,
            stderr: last_stderr,
        })
    }

    /// Spawn pipeline with stdin access only.
    pub fn spawn_with_stdin(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStdin>), Error> {
        let spawn = self.spawn_with_io()?;
        Ok((spawn.handle, spawn.stdin))
    }

    /// Spawn pipeline with stdout access only.
    pub fn spawn_with_stdout(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStdout>), Error> {
        let spawn = self.spawn_with_io()?;
        Ok((spawn.handle, spawn.stdout))
    }

    /// Spawn pipeline with stderr access only.
    pub fn spawn_with_stderr(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStderr>), Error> {
        let spawn = self.spawn_with_io()?;
        Ok((spawn.handle, spawn.stderr))
    }

    /// Spawn pipeline with both stdout and stderr access.
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
        let spawn = self.spawn_with_io()?;
        Ok((spawn.handle, spawn.stdout, spawn.stderr))
    }

    /// Run the pipeline and stream output to a Writer.
    /// This is more memory-efficient for large outputs.
    pub fn stream_to<W: Write>(mut self, mut writer: W) -> Result<(), Error> {
        // Extract input before spawning
        let input = self.input.take();
        let spawn = self.spawn_with_io()?;

        // Handle input in separate thread if provided
        let input_handle = match input {
            Some(CmdInput::Bytes(bytes)) => spawn.stdin.map(|mut stdin| {
                thread::spawn(move || {
                    use std::io::Write;
                    let _ = stdin.write_all(&bytes);
                    drop(stdin);
                })
            }),
            Some(CmdInput::Reader(mut reader)) => spawn.stdin.map(|stdin| {
                thread::spawn(move || {
                    use std::io::copy;
                    let mut stdin = stdin;
                    let _ = copy(&mut reader, &mut stdin);
                    drop(stdin);
                })
            }),
            None => None,
        };

        // Handle output in current thread
        if let Some(stdout) = spawn.stdout {
            use std::io::copy;
            copy(&mut BufReader::new(stdout), &mut writer).map_err(|e| Error {
                message: "Failed to copy pipeline output to writer".to_string(),
                source: Some(e),
            })?;
        }

        // Wait for input thread to complete if exists
        if let Some(handle) = input_handle {
            let _ = handle.join();
        }

        spawn.handle.wait()
    }

    /// Run the pipeline with both input Reader and output Writer.
    /// This is the most flexible method for streaming I/O.
    pub fn run_with_io<R: Read + Send + 'static, W: Write>(
        self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), Error> {
        let spawn = self.spawn_with_io()?;

        // Handle input in separate thread
        if let Some(mut stdin) = spawn.stdin {
            thread::spawn(move || {
                use std::io::copy;
                let _ = copy(&mut reader, &mut stdin);
            });
        }

        // Handle output in current thread
        if let Some(stdout) = spawn.stdout {
            use std::io::copy;
            copy(&mut BufReader::new(stdout), &mut writer).map_err(|e| Error {
                message: "Failed to copy pipeline output to writer".to_string(),
                source: Some(e),
            })?;
        }

        spawn.handle.wait()
    }

    fn execute_internal(mut self, capture_output: bool) -> Result<Vec<u8>, Error> {
        // Echo pipeline first if not suppressed
        let original_suppress = self.suppress_echo;
        if !original_suppress {
            self.echo_pipeline();
        }

        // Extract input before moving self
        let input = self.input.take();

        // Call spawn_with_io with echo suppressed to avoid double echo
        self.suppress_echo = true;
        let spawn = self.spawn_with_io()?;

        // Handle input if provided (for backward compatibility)
        let input_handle = match input {
            Some(CmdInput::Bytes(bytes)) => {
                spawn.stdin.map(|mut stdin| {
                    thread::spawn(move || {
                        use std::io::Write;
                        let _ = stdin.write_all(&bytes);
                        drop(stdin); // Close stdin to signal EOF
                    })
                })
            }
            Some(CmdInput::Reader(mut reader)) => {
                spawn.stdin.map(|stdin| {
                    thread::spawn(move || {
                        use std::io::copy;
                        let mut stdin = stdin;
                        let _ = copy(&mut reader, &mut stdin);
                        drop(stdin); // Close stdin to signal EOF
                    })
                })
            }
            None => None,
        };

        if capture_output {
            if let Some(stdout) = spawn.stdout {
                let mut output = Vec::new();
                let mut reader = BufReader::new(stdout);
                reader.read_to_end(&mut output).map_err(|e| Error {
                    message: "Failed to read stdout".to_string(),
                    source: Some(e),
                })?;

                // Wait for input thread to complete if exists
                if let Some(handle) = input_handle {
                    let _ = handle.join();
                }

                spawn.handle.wait()?;
                Ok(output)
            } else {
                // Wait for input thread to complete if exists
                if let Some(handle) = input_handle {
                    let _ = handle.join();
                }

                spawn.handle.wait()?;
                Ok(Vec::new())
            }
        } else {
            // Wait for input thread to complete if exists
            if let Some(handle) = input_handle {
                let _ = handle.join();
            }

            spawn.handle.wait()?;
            Ok(Vec::new())
        }
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
        if !crate::output::should_echo() {
            return;
        }

        let mut parts = Vec::new();

        // Add cmd prefix
        parts.push(format!("{BRIGHT_BLACK}cmd{BRIGHT_BLACK:#}"));

        for (i, (cmd, pipe_mode)) in self.connections.iter().enumerate() {
            if i > 0 {
                let pipe_symbol = match pipe_mode {
                    PipeMode::Stdout => "|",
                    PipeMode::Stderr => "|&",
                    PipeMode::Both => "|&&",
                };
                parts.push(format!("{MAGENTA}{pipe_symbol}{MAGENTA:#}"));
            }

            // Add current directory if set
            if let Some(current_dir) = &cmd.current_dir {
                let quoted_dir = Cmd::quote_argument(current_dir.as_os_str());
                parts.push(format!("{BRIGHT_BLUE}cd:{BRIGHT_BLUE:#}"));
                parts.push(format!(
                    "{UNDERLINE_BRIGHT_BLUE}{quoted_dir}{UNDERLINE_BRIGHT_BLUE:#}"
                ));
            }

            // Add environment variables
            for (key, val) in &cmd.envs {
                let quoted_key = Cmd::quote_argument(key);
                let quoted_val = Cmd::quote_argument(val);
                parts.push(format!("{BRIGHT_BLUE}env:{BRIGHT_BLUE:#}"));
                parts.push(format!(
                    "{UNDERLINE_BRIGHT_BLUE}{quoted_key}={quoted_val}{UNDERLINE_BRIGHT_BLUE:#}"
                ));
            }

            // Add program
            let quoted_program = Cmd::quote_argument(&cmd.program);
            parts.push(format!("{BOLD_CYAN}{quoted_program}{BOLD_CYAN:#}"));

            // Add arguments
            for arg in &cmd.args {
                let quoted_arg = Cmd::quote_argument(arg);
                parts.push(format!("{BOLD_UNDERLINE}{quoted_arg}{BOLD_UNDERLINE:#}"));
            }
        }

        eprintln!("{}", parts.join(" "));
    }
}
