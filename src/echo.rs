//! Echo module for printing messages to the console.

use crate::style::Style;

pub fn echo() -> Echo {
    Echo::default()
}

#[derive(Debug)]
#[must_use]
pub enum Echo {
    Quiet,
    Head,
    Tail,
}

impl Default for Echo {
    fn default() -> Self {
        Self::new()
    }
}

impl Echo {
    pub fn new() -> Self {
        match std::env::var_os("NO_ECHO").is_some() {
            true => Self::Quiet,
            false => Self::Head,
        }
    }

    pub fn quiet() -> Self {
        Self::Quiet
    }

    pub fn put(self, arg: impl std::fmt::Display) -> Self {
        match self {
            Self::Quiet => Self::Quiet,
            Self::Head => {
                print!("{arg}");
                Self::Tail
            }
            Self::Tail => {
                print!(" {arg}");
                Self::Tail
            }
        }
    }

    pub fn sput(self, arg: impl std::fmt::Display, style: Style) -> Self {
        self.put(format_args!("{style}{arg}{style:#}"))
    }

    pub fn end(self) {
        match self {
            Self::Quiet => {}
            _ => println!(),
        }
    }
}
