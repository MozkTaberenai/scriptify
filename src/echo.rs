//! Echo module for printing messages to the console.

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
                eprint!("{arg}");
                Self::Tail
            }
            Self::Tail => {
                eprint!(" {arg}");
                Self::Tail
            }
        }
    }

    pub fn sput(self, arg: impl std::fmt::Display, style: anstyle::Style) -> Self {
        self.put(format_args!("{style}{arg}{style:#}"))
    }

    pub fn end(self) {
        match self {
            Self::Quiet => {}
            _ => eprintln!(),
        }
    }
}

/// A macro to print to the standard output
#[macro_export]
macro_rules! echo {
    ($($arg:expr),* $(,)?) => {
        $crate::Echo::new()
            $(.put($arg))*
            .end();
    };
    () => {
        println!();
    };
}
