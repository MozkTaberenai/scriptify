use smallvec::{smallvec, SmallVec};

macro_rules! apply_macro {
    ($impl_macro:ident) => {
        $impl_macro!(bold, 1);
        $impl_macro!(dim, 2);
        $impl_macro!(italic, 3);
        $impl_macro!(underline, 4);

        $impl_macro!(black, 30);
        $impl_macro!(red, 31);
        $impl_macro!(green, 32);
        $impl_macro!(yellow, 33);
        $impl_macro!(blue, 34);
        $impl_macro!(magenta, 35);
        $impl_macro!(cyan, 36);
        $impl_macro!(bright_black, 90);
        $impl_macro!(bright_red, 91);
        $impl_macro!(bright_green, 92);
        $impl_macro!(bright_yellow, 93);
        $impl_macro!(bright_blue, 94);
        $impl_macro!(bright_magenta, 95);
        $impl_macro!(bright_cyan, 96);
        $impl_macro!(bright_white, 97);

        $impl_macro!(bg_black, 40);
        $impl_macro!(bg_red, 41);
        $impl_macro!(bg_green, 42);
        $impl_macro!(bg_yellow, 43);
        $impl_macro!(bg_blue, 44);
        $impl_macro!(bg_magenta, 45);
        $impl_macro!(bg_cyan, 46);
        $impl_macro!(bg_bright_black, 100);
        $impl_macro!(bg_bright_red, 101);
        $impl_macro!(bg_bright_green, 102);
        $impl_macro!(bg_bright_yellow, 103);
        $impl_macro!(bg_bright_blue, 104);
        $impl_macro!(bg_bright_magenta, 105);
        $impl_macro!(bg_bright_cyan, 106);
        $impl_macro!(bg_bright_white, 107);
    };
}

#[derive(Debug, Default, Clone)]
pub struct AnsiStyle {
    code: SmallVec<[u8; 8]>,
}

impl From<u8> for AnsiStyle {
    fn from(code: u8) -> Self {
        Self {
            code: smallvec![code],
        }
    }
}

macro_rules! impl_style_method {
    ($fn:ident, $code:literal) => {
        pub fn $fn(mut self) -> Self {
            self.code.push($code);
            self
        }
    };
}

impl AnsiStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply<T>(&self, target: T) -> AnsiStyled<T> {
        AnsiStyled {
            inner: target,
            style: self.clone(),
        }
    }

    apply_macro!(impl_style_method);
}

#[derive(Debug)]
pub struct AnsiStyled<T> {
    inner: T,
    style: AnsiStyle,
}

macro_rules! impl_styled_method {
    ($fn:ident, $code:literal) => {
        pub fn $fn(mut self) -> Self {
            self.style.code.push($code);
            self
        }
    };
}

impl<T> AnsiStyled<T>
where
    T: std::fmt::Display,
{
    pub fn new(inner: T, style: AnsiStyle) -> Self {
        Self { inner, style }
    }

    apply_macro!(impl_styled_method);
}

const RESET: &str = "\x1B[0m";

impl<T> std::fmt::Display for AnsiStyled<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::env::var_os("NO_COLOR").is_some() {
            true => write!(f, "{}", self.inner),
            false => {
                let mut iter = self.style.code.iter();
                if let Some(code) = iter.next() {
                    write!(f, "\x1B[{code}")?;
                } else {
                    return write!(f, "{}", self.inner);
                }
                for code in iter {
                    write!(f, ";{code}")?;
                }
                write!(f, "m{}{}", self.inner, RESET)
            }
        }
    }
}

macro_rules! impl_trait_method {
    ($fn:ident, $code:literal) => {
        fn $fn(self) -> AnsiStyled<Self> {
            AnsiStyled {
                inner: self,
                style: AnsiStyle::from($code),
            }
        }
    };
}

pub trait AnsiStyleExt: std::fmt::Display + Sized {
    apply_macro!(impl_trait_method);
}

impl<T> AnsiStyleExt for T where T: std::fmt::Display + Sized {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("a".red().to_string(), "\x1B[31ma\x1B[0m");
        assert_eq!("a".red().bold().to_string(), "\x1B[31;1ma\x1B[0m");
    }
}
