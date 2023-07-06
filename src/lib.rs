pub use std::ffi::{OsStr, OsString};
pub use std::io::{prelude::*, BufReader, BufWriter};
pub use std::path::{Path, PathBuf};

pub type AnyError = Box<dyn std::error::Error>;

mod ansi;
pub use ansi::AnsiStyleExt;

#[macro_use]
pub mod echo;
pub use echo::Echo;

#[macro_use]
mod cmd;
pub use cmd::Cmd;
pub mod env;
pub mod fs;
