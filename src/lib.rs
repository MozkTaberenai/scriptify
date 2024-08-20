//! # scriptant
//!
//! A Rust library designed for scripting. It provides a set of modules and functions that make it easier to perform common scripting tasks in Rust.
//!
//! The `cmd` module provides a set of functions for spawning and piping commands with echoes. It includes a `Command` struct that wraps around `std::process::Command` and includes additional functionality for echoing commands.
//!
//! The `fs` module provides wrappers around `std::fs` functions that echo the operation to the console. This can be useful for debugging and logging purposes.
//!
//! The `echo` module provides a set of functions for printing messages to the console. It includes a `Echo` struct that can be used to build and format console messages.
//!
//! The `style` module provides a set of functions for styling console output. It includes a `Style` struct that can be used to apply various styles to console output.
//!
//! The `scriptant` library also includes a set of macros for creating new commands and printing to the standard output.
//!
//! The `scriptant` library is designed to be easy to use and flexible, making it a great choice for scripting tasks in Rust.
// #![warn(missing_docs)]

#[doc(no_inline)]
pub use std::ffi::{OsStr, OsString};
#[doc(no_inline)]
pub use std::io::{prelude::*, BufReader, BufWriter};
#[doc(no_inline)]
pub use std::path::{Path, PathBuf};

pub type AnyError = Box<dyn std::error::Error>;
pub type Result<T, E = AnyError> = std::result::Result<T, E>;

pub mod cmd;
#[doc(inline)]
pub use cmd::*;

pub mod fs;

pub mod echo;
pub use echo::echo;

pub mod style;
pub use style::style;

/// A macro to create a new command
#[macro_export]
macro_rules! cmd {
    ($program:expr) => {
        $crate::cmd::Command::new($program)
    };
    ($program:expr, $($arg:expr),* $(,)?) => {
        $crate::cmd::Command::new($program)$(.arg($arg))*
    };
}

/// A macro to print to the standard output
#[macro_export]
macro_rules! echo {
    ($($arg:expr),* $(,)?) => {
        $crate::echo::Echo::new()
            $(.put($arg))*
            .end();
    };
    () => {
        println!();
    };
}
