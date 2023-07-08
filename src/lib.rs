pub use std::ffi::{OsStr, OsString};
pub use std::io::{prelude::*, BufReader, BufWriter};
pub use std::path::{Path, PathBuf};

pub type AnyError = Box<dyn std::error::Error>;

pub mod ansi;
pub use ansi::StyleExt;

// #[macro_use]
mod echo;
use echo::Echo;

#[macro_use]
mod cmd;
pub use cmd::Cmd;

pub mod env;
pub mod fs;
