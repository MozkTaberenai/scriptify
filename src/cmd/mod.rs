//! A module for spawning and piping commands with echoes.
//!
//! This module provides a convenient way to spawn system commands and pipe their outputs. It also echoes the commands to the console for easy tracking.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use scriptant::*;
//!
//! cmd!("ls", "/").run().unwrap();
//! ```
//!
//! The above example will run the `ls` command with the `/` argument, and print the command to the console. This is useful for listing the contents of the root directory.
//!
//! Piping commands:
//!
//! ```
//! use scriptant::*;
//!
//! cmd!("ls", "/").pipe(cmd!("grep", "bin")).run().unwrap();
//! ```
//!
//! The above example will run the `ls` command with the `/` argument, pipe the output to the `grep` command with the `bin` argument, and print the commands to the console. This is useful for searching for specific files or directories.

mod child;
mod command;
mod err;
mod handle;
mod io;
mod pipeline;
mod spawn;
mod status;

// Publicly re-exporting the modules for external use.
pub use command::*;
pub use err::*;
pub use handle::*;
pub use io::*;
pub use pipeline::*;
pub use spawn::*;
pub use status::*;

// Module for testing the functionality of the scriptant library.
#[cfg(test)]
mod test;
