pub mod ansi;
pub mod cmd;
pub mod confirm;
pub mod echo;
pub mod env;
pub mod fs;

pub use ansi::StyleExt;
pub use cmd::{Pipe, ReadSpawn, ReadSpawnExt, Spawn, WriteReadSpawn, WriteSpawn};

pub mod prelude {
    pub use std::ffi::{OsStr, OsString};
    pub use std::io::{prelude::*, BufReader, BufWriter};
    pub use std::path::{Path, PathBuf};
    pub type AnyError = Box<dyn std::error::Error>;
}
pub use prelude::*;
