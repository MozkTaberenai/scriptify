mod child;
mod command;
mod error;
mod handle;
mod io;
mod pipe;
mod pipeline;
mod spawn;
mod status;

use child::*;
pub use command::*;
pub use error::*;
pub use handle::*;
pub use io::*;
pub use pipe::*;
pub use pipeline::*;
pub use spawn::*;
pub use status::*;

fn echo(quiet: bool) -> crate::echo::Echo {
    use crate::ansi::StyleExt;
    let mut echo = crate::echo::Echo::default();
    if quiet {
        echo.quiet();
    }
    echo.put("cmd".bright_black());
    echo
}

#[macro_export]
macro_rules! cmd {
    ($program:expr) => {
        $crate::cmd::Command::new($program)
    };
    ($program:expr, $($arg:expr),* $(,)?) => {
        $crate::cmd::Command::new($program)$(.arg($arg))*
    };
}

#[cfg(test)]
mod test;
