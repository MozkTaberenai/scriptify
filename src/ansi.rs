#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
    #[default]
    None = 0,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

impl Color {
    fn fg_code(&self) -> u8 {
        *self as u8
    }
    fn bg_code(&self) -> u8 {
        *self as u8 + 10
    }
}

#[derive(Debug, Default)]
struct AnsiStyle {
    bold: bool,      // -> code:1
    dim: bool,       // -> code:2
    italic: bool,    // -> code:3
    underline: bool, // -> code:4
    fg: Color,       // -> code: 30-37, 90-97
    bg: Color,       // -> code: 40-47, 100-107
}

macro_rules! impl_style_method {
    ($n:ident) => {
        fn $n(&mut self) -> &mut Self {
            self.$n = true;
            self
        }
    };
}

macro_rules! impl_color_method {
    ($fg:ident, $bg:ident, $c:path) => {
        fn $fg(&mut self) -> &mut Self {
            self.fg = $c;
            self
        }

        fn $bg(&mut self) -> &mut Self {
            self.bg = $c;
            self
        }
    };
}

impl AnsiStyle {
    fn codes(&self) -> impl Iterator<Item = u8> {
        let mut code = [0u8; 6];
        if self.bold {
            code[0] = 1;
        }
        if self.dim {
            code[1] = 2;
        }
        if self.italic {
            code[2] = 3;
        }
        if self.underline {
            code[3] = 4;
        }
        if self.fg != Color::None {
            code[4] = self.fg.fg_code();
        }
        if self.bg != Color::None {
            code[5] = self.bg.bg_code();
        }
        code.into_iter().filter(|code| *code != 0)
    }

    impl_style_method!(bold);
    impl_style_method!(dim);
    impl_style_method!(italic);
    impl_style_method!(underline);

    impl_color_method!(black, bg_black, Color::Black);
    impl_color_method!(red, bg_red, Color::Red);
    impl_color_method!(green, bg_green, Color::Green);
    impl_color_method!(yellow, bg_yellow, Color::Yellow);
    impl_color_method!(blue, bg_blue, Color::Blue);
    impl_color_method!(magenta, bg_magenta, Color::Magenta);
    impl_color_method!(cyan, bg_cyan, Color::Cyan);
    impl_color_method!(white, bg_white, Color::White);
    impl_color_method!(bright_black, bg_bright_black, Color::BrightBlack);
    impl_color_method!(bright_red, bg_bright_red, Color::BrightRed);
    impl_color_method!(bright_green, bg_bright_green, Color::BrightGreen);
    impl_color_method!(bright_yellow, bg_bright_yellow, Color::BrightYellow);
    impl_color_method!(bright_blue, bg_bright_blue, Color::BrightBlue);
    impl_color_method!(bright_magenta, bg_bright_magenta, Color::BrightMagenta);
    impl_color_method!(bright_cyan, bg_bright_cyan, Color::BrightCyan);
    impl_color_method!(bright_white, bg_bright_white, Color::BrightWhite);
}

#[derive(Debug)]
pub struct AnsiStyled<T> {
    target: T,
    style: AnsiStyle,
}

macro_rules! impl_all {
    ($impl_item:ident) => {
        $impl_item!(bold);
        $impl_item!(dim);
        $impl_item!(italic);
        $impl_item!(underline);
        $impl_item!(black);
        $impl_item!(bg_black);
        $impl_item!(red);
        $impl_item!(bg_red);
        $impl_item!(green);
        $impl_item!(bg_green);
        $impl_item!(yellow);
        $impl_item!(bg_yellow);
        $impl_item!(blue);
        $impl_item!(bg_blue);
        $impl_item!(magenta);
        $impl_item!(bg_magenta);
        $impl_item!(cyan);
        $impl_item!(bg_cyan);
        $impl_item!(white);
        $impl_item!(bg_white);
        $impl_item!(bright_black);
        $impl_item!(bg_bright_black);
        $impl_item!(bright_red);
        $impl_item!(bg_bright_red);
        $impl_item!(bright_green);
        $impl_item!(bg_bright_green);
        $impl_item!(bright_yellow);
        $impl_item!(bg_bright_yellow);
        $impl_item!(bright_blue);
        $impl_item!(bg_bright_blue);
        $impl_item!(bright_magenta);
        $impl_item!(bg_bright_magenta);
        $impl_item!(bright_cyan);
        $impl_item!(bg_bright_cyan);
        $impl_item!(bright_white);
        $impl_item!(bg_bright_white);
    };
}

macro_rules! impl_styled_method {
    ($fn:ident) => {
        pub fn $fn(mut self) -> Self {
            self.style.$fn();
            self
        }
    };
}

impl<T> AnsiStyled<T>
where
    T: std::fmt::Display,
{
    impl_all!(impl_styled_method);
}

const RESET: &str = "\x1B[0m";

impl<T> std::fmt::Display for AnsiStyled<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::env::var_os("NO_COLOR").is_some() {
            true => write!(f, "{}", self.target),
            false => {
                let mut codes = self.style.codes();
                if let Some(code) = codes.next() {
                    write!(f, "\x1B[{code}")?;
                    for code in codes {
                        write!(f, ";{code}")?;
                    }
                    write!(f, "m{}{}", self.target, RESET)
                } else {
                    write!(f, "{}", self.target)
                }
            }
        }
    }
}

macro_rules! impl_trait_method {
    ($fn:ident) => {
        fn $fn(self) -> AnsiStyled<Self> {
            let mut style = AnsiStyle::default();
            style.$fn();
            AnsiStyled {
                target: self,
                style,
            }
        }
    };
}

pub trait AnsiStyleExt: std::fmt::Display + Sized {
    impl_all!(impl_trait_method);
}

impl<T> AnsiStyleExt for T where T: std::fmt::Display + Sized {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("a".red().to_string(), "\x1B[31ma\x1B[0m");
        assert_eq!("a".red().bold().to_string(), "\x1B[1;31ma\x1B[0m");
    }
}
