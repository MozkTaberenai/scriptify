pub use std::ffi::{OsStr, OsString};
pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};
pub use std::path::{Path, PathBuf};
pub use std::process::Stdio;

pub type AnyError = Box<dyn std::error::Error>;

mod ansi;
pub use ansi::{style, AnsiStyle, AnsiStyleExt, AnsiStyled};

#[macro_use]
mod echo;
pub use echo::Echo;

#[macro_use]
mod cmd;
pub use cmd::Cmd;
pub mod env;
pub mod fs;

pub fn exit(code: i32) -> ! {
    if code != 0 {
        echo_err!("Exit with code:", code);
    }
    std::process::exit(code);
}
