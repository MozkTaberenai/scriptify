// use crate::AnsiStyleExt;
use std::borrow::Cow;

pub enum Echo {
    Null,
    Stdout(Option<Cow<'static, str>>),
}

impl Default for Echo {
    fn default() -> Self {
        match std::env::var_os("NO_ECHO").is_some() {
            true => Self::Null,
            false => Self::Stdout(None),
        }
    }
}

impl Echo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quiet(&mut self) -> &mut Self {
        *self = Self::Null;
        self
    }

    pub fn put(&mut self, arg: impl std::fmt::Display) -> &mut Self {
        match self {
            Self::Null => {}
            Self::Stdout(p) => {
                if let Some(p) = p.take() {
                    print!("{p}");
                }
                print!("{arg}");
                p.replace(" ".into());
            }
        }
        self
    }

    pub fn end(self) {
        match self {
            Self::Null => {}
            Self::Stdout(_) => println!(),
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
macro_rules! err {
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.put($arg.red());)*
        echo.end();
    }};
}

#[macro_export]
macro_rules! wrn {
    ($($arg:expr),* $(,)?) => {{
        let mut echo = Echo::new();
        $(echo.put($arg.yellow());)*
        echo.end();
    }};
}
