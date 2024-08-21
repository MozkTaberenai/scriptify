//! File system operations.
//!
//! This module provides wrappers around `std::fs` functions that echo the operation to the console.
//! It offers a set of functions for working with files and directories.
//!
//! # Examples
//!
//! ```
//! use scriptant::*;
//!
//! fn main() -> std::io::Result<()> {
//!     fs::write("foo.txt", "Hello, world!")?;
//!     fs::read_to_string("foo.txt")?;
//!     fs::remove_file("foo.txt")?;
//!     Ok(())
//! }
//! ```
//!
//! For more information on the behavior of these functions, see the documentation for the corresponding
//! functions in [`std::fs`].

use crate::{echo::Echo, style, style::Style};
use std::path::Path;

const BRIGHT_BLACK: Style = style().bright_black();
const BOLD_UNDERLINE: Style = style().bold().underline();
const BOLD_CYAN: Style = style().bold().cyan();

fn echo(op: impl std::fmt::Display) -> Echo {
    crate::echo().sput("fs", BRIGHT_BLACK).sput(op, BOLD_CYAN)
}

/// Rename a file or directory to a new name, replacing the original file if `to` already exists.
///
/// This is a wrapper around [`std::fs::rename`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::rename("old.txt", "new.txt")?;
///     Ok(())
/// }
/// ```
pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    echo("rename")
        .sput(from.to_string_lossy(), BOLD_UNDERLINE)
        .put("->")
        .sput(to.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::rename(from, to)
}

/// Copy the contents of one file to another.
///
/// This is a wrapper around [`std::fs::copy`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::copy("foo.txt", "bar.txt")?;
///     Ok(())
/// }
/// ```
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<u64> {
    let from = from.as_ref();
    let to = to.as_ref();
    echo("copy")
        .sput(from.to_string_lossy(), BOLD_UNDERLINE)
        .put("->")
        .sput(to.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::copy(from, to)
}

/// Create a new hard link to a file.
///
/// This is a wrapper around [`std::fs::hard_link`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::hard_link("old.txt", "new.txt")?;
///     Ok(())
/// }
/// ```
pub fn hard_link(original: impl AsRef<Path>, link: impl AsRef<Path>) -> std::io::Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();
    echo("hard_link")
        .sput(original.to_string_lossy(), BOLD_UNDERLINE)
        .put("->")
        .sput(link.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::hard_link(original, link)
}

/// Create a new, empty directory at the provided path.
///
/// This is a wrapper around [`std::fs::create_dir`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::create_dir("./some/dir")?;
///     Ok(())
/// }
/// ```
pub fn create_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir(path)
}

/// Recursively create a directory and all of its parent components if they are missing.
///
/// This is a wrapper around [`std::fs::create_dir_all`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::create_dir_all("./some/dir")?;
///     Ok(())
/// }
/// ```
pub fn create_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir_all(path)
}

/// Given a path, query the file system to get information about a file, directory, etc.
///
/// This is a wrapper around [`std::fs::metadata`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     let metadata = fs::metadata("foo.txt")?;
///     println!("{:?}", metadata.file_type());
///     Ok(())
/// }
/// ```
pub fn metadata(path: impl AsRef<Path>) -> std::io::Result<std::fs::Metadata> {
    let path = path.as_ref();
    echo("metadata")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::metadata(path)
}

/// Returns an iterator over the entries within a directory.
///
/// This is a wrapper around [`std::fs::read_dir`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     for entry in fs::read_dir(".")? {
///         let entry = entry?;
///         println!("{:?}", entry.path());
///     }
///     Ok(())
/// }
/// ```
pub fn read_dir(path: impl AsRef<Path>) -> std::io::Result<std::fs::ReadDir> {
    let path = path.as_ref();
    echo("read_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_dir(path)
}

/// Read the entire contents of a file into a bytes vector.
///
/// This is a wrapper around [`std::fs::read`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     let contents = fs::read("foo.txt")?;
///     println!("{:?}", contents);
///     Ok(())
/// }
/// ```
pub fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    let path = path.as_ref();
    echo("read")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read(path)
}

/// Read the entire contents of a file into a string.
///
/// This is a wrapper around [`std::fs::read_to_string`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     let contents = fs::read_to_string("foo.txt")?;
///     println!("{}", contents);
///     Ok(())
/// }
/// ```
pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    let path = path.as_ref();
    echo("read_to_string")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_to_string(path)
}

/// Write a slice as the entire contents of a file.
///
/// This is a wrapper around [`std::fs::write`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::write("foo.txt", b"Hello, world!")?;
///     Ok(())
/// }
/// ```
pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    let path = path.as_ref();
    let contents = contents.as_ref();
    echo("write")
        .put(format_args!("{} bytes", contents.len()))
        .put("->")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::write(path, contents)
}

/// Removes an empty directory.
///
/// This is a wrapper around [`std::fs::remove_dir`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::remove_dir("./some/dir")?;
///     Ok(())
/// }
/// ```
pub fn remove_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir(path)
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
///
/// This is a wrapper around [`std::fs::remove_dir_all`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::remove_dir_all("./some/dir")?;
///     Ok(())
/// }
/// ```
pub fn remove_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir_all(path)
}

/// Removes a file from the filesystem.
///
/// This is a wrapper around [`std::fs::remove_file`] that echoes the operation to the console.
///
/// # Examples
///
/// ```no_run
/// use scriptant::*;
///
/// fn main() -> std::io::Result<()> {
///     fs::remove_file("foo.txt")?;
///     Ok(())
/// }
/// ```
pub fn remove_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_file")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_file(path)
}
