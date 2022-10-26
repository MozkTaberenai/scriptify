pub struct Echo {
    stream: Stream,
    disabled: bool,
    count: usize,
}

enum Stream {
    Stdout,
    Stderr,
}

impl Default for Echo {
    fn default() -> Self {
        let stream = match std::env::var_os("ECHO_STDERR").is_some() {
            true => Stream::Stderr,
            false => Stream::Stdout,
        };
        let disabled = std::env::var_os("NO_ECHO").is_some();
        Self {
            stream,
            disabled,
            count: 0,
        }
    }
}

macro_rules! p {
    ($out:expr, $($arg:tt)*) => {
        match $out {
            Stream::Stdout => print!($($arg)*),
            Stream::Stderr => eprint!($($arg)*),
        }
    };
}

impl Echo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.disabled = true;
        self
    }

    pub fn out(&mut self, arg: impl std::fmt::Display) -> &mut Self {
        if self.disabled {
            return self;
        }

        match self.count {
            0 => p!(self.stream, "{}", arg),
            _ => p!(self.stream, " {}", arg),
        }
        self.count += 1;

        self
    }

    pub fn end(self) {
        if self.disabled {
            return;
        }

        match self.stream {
            Stream::Stdout => println!(),
            Stream::Stderr => eprintln!(),
        }
    }
}

#[macro_export]
macro_rules! echo {
    (!, $($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.out($arg.yellow());)*
        echo.end();
    }};
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.out($arg);)*
        echo.end();
    }};
}
#[macro_export]
macro_rules! echo_err {
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.out($arg.yellow());)*
        echo.end();
    }};
}

use crate::AnsiStyleExt;

pub fn prefix(tag: &'static str) -> String {
    format!("{} {}", tag.bright_black(), ">".green())
}

pub fn error<E: std::error::Error>(err: E) -> E {
    echo_err!(&err);
    err
}
