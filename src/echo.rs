use crate::AnsiStyleExt;

pub struct EchoContext {
    stream: Stream,
    disabled: bool,
    tag: Option<&'static str>,
}

#[derive(Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

use Stream::*;

impl EchoContext {
    pub fn new(tag: &'static str) -> Self {
        let stream = match std::env::var_os("ECHO_STDERR").is_some() {
            true => Stderr,
            false => Stdout,
        };

        let disabled = std::env::var_os("NO_ECHO").is_some();

        let tag = Some(tag);

        Self {
            stream,
            disabled,
            tag,
        }
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.disabled = true;
        self
    }

    pub fn put(&mut self, arg: impl std::fmt::Display) -> &mut Self {
        if self.disabled {
            return self;
        }

        if let Some(tag) = self.tag.take() {
            let tag = format!("{} {}", tag.bright_black(), "%".green());
            match self.stream {
                Stdout => print!("{tag}"),
                Stderr => eprint!("{tag}"),
            }
        }

        match self.stream {
            Stdout => print!(" {arg}"),
            Stderr => eprint!(" {arg}"),
        }

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
    ($tag:expr, $($arg:expr),* $(,)?) => {{
        let mut echo = EchoContext::new($tag);
        $(echo.put($arg);)*
        echo.end();
    }};
}
