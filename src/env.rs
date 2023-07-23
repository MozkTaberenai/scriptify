use crate::ansi::StyleExt;
use std::ffi::OsStr;
use std::path::Path;

macro_rules! echo {
    ($($arg:expr),* $(,)?) => {
        $crate::echo!("env".bright_black(), $($arg,)*);
    };
}

pub fn set_current_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    echo!(
        "set_current_dir".bold().cyan(),
        path.to_string_lossy().bold().underline(),
    );
    std::env::set_current_dir(path)
}

pub fn set_var(key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) {
    let key = key.as_ref();
    let value = value.as_ref();
    echo!(
        "set_var".bold().cyan(),
        key.to_string_lossy().bold().underline(),
        "=".bright_black(),
        value.to_string_lossy().bold().underline(),
    );
    std::env::set_var(key, value);
}
