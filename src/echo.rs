use crate::AnsiStyleExt;

pub struct Echo {
    stream: Stream,
    disabled: bool,
    count: usize,
}

#[derive(Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

use Stream::*;

impl Default for Echo {
    fn default() -> Self {
        let stream = match std::env::var_os("ECHO_STDERR").is_some() {
            true => Stderr,
            false => Stdout,
        };

        let disabled = std::env::var_os("NO_ECHO").is_some();

        Self {
            stream,
            disabled,
            count: 0,
        }
    }
}

impl Echo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.disabled = true;
        self
    }

    pub fn put(&mut self, arg: impl std::fmt::Display) -> &mut Self {
        if self.disabled {
            return self;
        }

        match (self.count, self.stream) {
            (0, Stdout) => print!("{}", arg),
            (0, Stderr) => eprint!("{}", arg),
            (_, Stdout) => print!(" {}", arg),
            (_, Stderr) => eprint!(" {}", arg),
        }

        self.count += 1;

        self
    }

    pub fn end(self) {
        if self.disabled {
            return;
        }

        match self.stream {
            Stdout => println!(),
            Stderr => eprintln!(),
        }
    }
}

#[macro_export]
macro_rules! echo {
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.put($arg);)*
        echo.end();
    }};
}

#[macro_export]
macro_rules! echo_err {
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.put($arg.yellow());)*
        echo.end();
    }};
}

pub fn prefix(tag: &'static str) -> String {
    format!("{} {}", tag.bright_black(), "%".green())
}

pub trait EchoErrExt {
    fn echo_err(self) -> Self;
}

impl<T, E: std::fmt::Display> EchoErrExt for std::result::Result<T, E> {
    fn echo_err(self) -> Self {
        if let Err(ref err) = self {
            echo_err!(err);
        }
        self
    }
}
