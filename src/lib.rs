//! # scriptant
//!
//! This Rust crate is designed for scripting tasks, providing easy-to-use functions and macros for common operations. Let's explore its usage through examples.
//!
//! # Examples
//!
//! ## Command Execution
//!
//! The `cmd!` macro allows you to execute shell commands:
//!
//! ```no_run
//! use scriptant::*;
//!
//! // Simple command execution
//! cmd!("echo", "Hello,", "World!").run()?;
//!
//! // Command with multiple arguments using an iterator
//! cmd!("echo").args((0..5).map(|n| n.to_string())).run()?;
//!
//! // Command with spaces in arguments
//! cmd!("echo", "arg with  space").run()?;
//!
//! // Setting environment variables
//! cmd!("echo", "with", "env").env("AAA", "aaa").run()?;
//!
//! // Changing current directory
//! cmd!("ls", "-alF").current_dir("src").run()?;
//! # Ok::<(), Error>(())
//! ```
//!
//! ## Command Piping
//!
//! The crate supports command piping:
//!
//! ```no_run
//! use scriptant::*;
//!
//! // Piping multiple commands
//! cmd!("date")
//!     .pipe(cmd!("rev"))
//!     .pipe(cmd!("tr", "[:upper:]", "[:lower:]"))
//!     .run()?;
//!
//! // Reading piped output
//! let out = cmd!("echo", "pipe input").read_to_string()?;
//! echo!("pipe output:", out.trim());
//!
//! // Piping from byte slice
//! b"pipe input from slice\n"
//!     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
//!     .run()?;
//!
//! // Piping from string
//! let pipe_input = "abcde";
//! echo!("pipe input:", pipe_input);
//! pipe_input
//!     .as_bytes()
//!     .pipe(cmd!("rev"))
//!     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
//!     .run()?;
//! # Ok::<(), Error>(())
//! ```
//!
//! ## Writing to Command's Stdin
//!
//! You can write directly to a command's stdin:
//!
//! ```no_run
//! use scriptant::*;
//!
//! let (mut stdin, handle) = cmd!("tr", "[:lower:]", "[:upper:]").write_spawn()?;
//! writeln!(stdin, "x")?;
//! stdin.write_all(b"y\n")?;
//! stdin.write_all("z\n".as_bytes())?;
//! drop(stdin);
//! handle.wait()?;
//! # Ok::<(), Error>(())
//! ```
//! This will output:
//! ```plaintext
//! X
//! Y
//! Z
//! ```
//!
//! ## File Operations
//!
//! The `fs` module provides wrappers around [`std::fs`] operations:
//!
//! ```no_run
//! use scriptant::*;
//!
//! fs::create_dir("tmp")?;
//! fs::write("tmp/a.txt", "abc")?;
//! fs::copy("tmp/a.txt", "tmp/b.txt")?;
//! fs::hard_link("tmp/a.txt", "tmp/h.txt")?;
//! fs::rename("tmp/a.txt", "tmp/c.txt")?;
//! fs::create_dir_all("tmp/d/e")?;
//! fs::remove_file("tmp/b.txt")?;
//! fs::remove_dir("tmp/d/e")?;
//! fs::remove_dir_all("tmp")?;
//! # Ok::<(), Error>(())
//! ```
//!
//! These examples demonstrate the core functionality of the `scriptant` library, showcasing its simplicity and power for various scripting tasks in Rust.
//!

#[doc(no_inline)]
pub use std::ffi::{OsStr, OsString};
#[doc(no_inline)]
pub use std::io::{prelude::*, BufReader, BufWriter};
#[doc(no_inline)]
pub use std::path::{Path, PathBuf};

pub mod cmd;
pub use cmd::*;

pub mod fs;

pub mod echo;
pub use echo::Echo;

pub mod color;

// style presets
mod style;
