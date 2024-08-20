//! This module provides wrappers around `std::fs` functions that echo the operation to the console.
//!
//! # Examples
//!
//! ```
//! use scriptant::*;
//! fs::rename("old_name.txt", "new_name.txt");
//! ```
//!
//! This will rename the file `old_name.txt` to `new_name.txt` and print the operation to the console.
//!
//! # Note
//!
//! All functions in this module return `std::io::Result`, which means that they will return an error
//! if the operation fails for any reason (for example, if the file does not exist).

use crate::{echo::Echo, style, style::Style};
use std::path::Path;

const BRIGHT_BLACK: Style = style().bright_black();
const BOLD_UNDERLINE: Style = style().bold().underline();
const BOLD_CYAN: Style = style().bold().cyan();

fn echo(op: impl std::fmt::Display) -> Echo {
    crate::echo().sput("fs", BRIGHT_BLACK).sput(op, BOLD_CYAN)
}

/// Renames a file or directory, as specified by `from`, to `to`.
///
/// This function will rename the file or directory at the path specified by `from` to the path specified by `to`.
/// If the `from` path does not exist, the function will return an error. If the `to` path already exists, it will be overwritten.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::rename("old_name.txt", "new_name.txt");
/// ```
///
/// This will rename the file `old_name.txt` to `new_name.txt` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `from` path does not exist.
/// * The user lacks permissions to view or modify the `from` or `to` path.
///
/// # Note
///
/// This function relies on [`std::fs::rename`] from the standard library.
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

/// Copies a file from `from` to `to`.
///
/// This function will copy the file at the path specified by `from` to the path specified by `to`.
/// If the `from` path does not exist, the function will return an error. If the `to` path already exists, it will be overwritten.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::copy("source.txt", "destination.txt");
/// ```
///
/// This will copy the file `source.txt` to `destination.txt` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `from` path does not exist.
/// * The user lacks permissions to view or modify the `from` or `to` path.
///
/// # Note
///
/// This function relies on [`std::fs::copy`] from the standard library.
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

/// Creates a hard link from `original` to `link`.
///
/// This function will create a hard link from the file at the path specified by `original` to the path specified by `link`.
/// If the `original` path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::hard_link("original.txt", "link.txt");
/// ```
///
/// This will create a hard link from the file `original.txt` to `link.txt` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `original` path does not exist.
/// * The user lacks permissions to view or modify the `original` or `link` path.
///
/// # Note
///
/// This function relies on [`std::fs::hard_link`] from the standard library.
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

/// Creates a new directory at the path specified by `path`.
///
/// This function will create a new directory at the path specified by `path`.
/// If the directory already exists, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::create_dir("new_directory");
/// ```
///
/// This will create a new directory named `new_directory` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` already exists.
/// * The user lacks permissions to create the directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::create_dir`] from the standard library.
pub fn create_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir(path)
}

/// Creates a new directory and all its parent directories if they do not exist.
///
/// This function will create a new directory and all its parent directories if they do not exist.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::create_dir_all("a/b/c");
/// ```
///
/// This will create a new directory at `a/b/c` and all its parent directories if they do not exist, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The user lacks permissions to create the directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::create_dir_all`] from the standard library.
pub fn create_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir_all(path)
}

/// Retrieves metadata for the file or directory at `path`.
///
/// This function will retrieve metadata for the file or directory at `path`.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::metadata("file.txt");
/// ```
///
/// This will retrieve metadata for the file `file.txt` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to view the metadata of the file or directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::metadata`] from the standard library.
pub fn metadata(path: impl AsRef<Path>) -> std::io::Result<std::fs::Metadata> {
    let path = path.as_ref();
    echo("metadata")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::metadata(path)
}

/// Reads the directory at `path` and returns an iterator to the entries.
///
/// This function will read the directory at `path` and return an iterator to the entries.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::read_dir("directory");
/// ```
///
/// This will read the directory `directory` and return an iterator to the entries, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to read the directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::read_dir`] from the standard library.
pub fn read_dir(path: impl AsRef<Path>) -> std::io::Result<std::fs::ReadDir> {
    let path = path.as_ref();
    echo("read_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_dir(path)
}

/// Reads the file at `path` and returns its contents as a vector of bytes.
///
/// This function will read the file at `path` and return its contents as a vector of bytes.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::read("file.txt");
/// ```
///
/// This will read the file `file.txt` and return its contents as a vector of bytes, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to read the file at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::read`] from the standard library.
pub fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    let path = path.as_ref();
    echo("read")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read(path)
}

/// Reads the file at `path` and returns its contents as a string.
///
/// This function will read the file at `path` and return its contents as a string.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::read_to_string("file.txt");
/// ```
///
/// This will read the file `file.txt` and return its contents as a string, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to read the file at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::read_to_string`] from the standard library.
pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    let path = path.as_ref();
    echo("read_to_string")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_to_string(path)
}

/// Writes `contents` to the file at `path`.
///
/// This function will write `contents` to the file at `path`.
/// If the path does not exist, the function will create it.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::write("file.txt", b"Hello, world!");
/// ```
///
/// This will write the byte string `b"Hello, world!"` to the file `file.txt`, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The user lacks permissions to write to the file at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::write`] from the standard library.
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

/// Removes the directory at `path`.
///
/// This function will remove the directory at `path`.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::remove_dir("directory");
/// ```
///
/// This will remove the directory `directory` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to remove the directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::remove_dir`] from the standard library.
pub fn remove_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir(path)
}

/// Removes the directory at `path` and all its contents.
///
/// This function will remove the directory at `path` and all its contents.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::remove_dir_all("directory");
/// ```
///
/// This will remove the directory `directory` and all its contents, and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to remove the directory at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::remove_dir_all`] from the standard library.
pub fn remove_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir_all(path)
}

/// Removes the file at `path`.
///
/// This function will remove the file at `path`.
/// If the path does not exist, the function will return an error.
///
/// This function also prints the operation to the console.
///
/// # Examples
///
/// ```
/// use scriptant::*;
/// fs::remove_file("file.txt");
/// ```
///
/// This will remove the file `file.txt` and print the operation to the console.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
///
/// * The `path` does not exist.
/// * The user lacks permissions to remove the file at the specified `path`.
///
/// # Note
///
/// This function relies on [`std::fs::remove_file`] from the standard library.
pub fn remove_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_file")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_file(path)
}
