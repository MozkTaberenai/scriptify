// use crate::echo::Echo;
// use anstyle::{AnsiColor, Color, Style};
use crate::style;
use std::path::Path;

const BRIGHT_BLACK: crate::echo::Style = style().bright_black();
const BOLD_UNDERLINE: crate::echo::Style = style().bold().underline();
const BOLD_CYAN: crate::echo::Style = style().bold().cyan();

fn echo(op: impl std::fmt::Display) -> crate::Echo {
    crate::echo().sput("fs", BRIGHT_BLACK).sput(op, BOLD_CYAN)
}

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

pub fn create_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir(path)
}

pub fn create_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("create_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::create_dir_all(path)
}

pub fn metadata(path: impl AsRef<Path>) -> std::io::Result<std::fs::Metadata> {
    let path = path.as_ref();
    echo("metadata")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::metadata(path)
}

pub fn read_dir(path: impl AsRef<Path>) -> std::io::Result<std::fs::ReadDir> {
    let path = path.as_ref();
    echo("read_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_dir(path)
}

pub fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    let path = path.as_ref();
    echo("read")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read(path)
}

pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    let path = path.as_ref();
    echo("read_to_string")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::read_to_string(path)
}

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

pub fn remove_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir(path)
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_dir_all")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_dir_all(path)
}

pub fn remove_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo("remove_file")
        .sput(path.to_string_lossy(), BOLD_UNDERLINE)
        .end();
    std::fs::remove_file(path)
}
