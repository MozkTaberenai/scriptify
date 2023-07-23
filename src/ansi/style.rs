use super::CSI;

const RESET: u8 = 0;
const BOLD: u8 = 1;
const DIMMED: u8 = 2;
const ITALIC: u8 = 3;
const UNDERLINE: u8 = 4;

const FOREGROUND: u8 = 30;
const BACKGROUND: u8 = 40;
const BRIGHT: u8 = 60;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Ansi8Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}
use Ansi8Color::*;

impl Ansi8Color {
    fn code(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug)]
enum Ansi16Color {
    Standard(Ansi8Color),
    Bright(Ansi8Color),
}
use Ansi16Color::*;

impl Ansi16Color {
    fn code(&self) -> u8 {
        match self {
            Standard(ansi8) => ansi8.code(),
            Bright(ansi8) => ansi8.code() + BRIGHT,
        }
    }
}

#[derive(Debug)]
enum Color {
    Ansi16(Ansi16Color),
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Default)]
pub struct Style {
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    foreground: Option<Color>,
    background: Option<Color>,
}

pub fn style() -> Style {
    Style::default()
}

macro_rules! impl_boolean {
    ($n:ident) => {
        pub fn $n(mut self) -> Self {
            self.$n = true;
            self
        }
    };
}

macro_rules! impl_color {
    ($fn:ident, $p:ident, $color:expr) => {
        pub fn $fn(mut self) -> Self {
            self.$p = Some($color);
            self
        }
    };
}

impl Style {
    impl_boolean!(bold);
    impl_boolean!(dimmed);
    impl_boolean!(italic);
    impl_boolean!(underline);

    impl_color!(black, foreground, Color::Ansi16(Standard(Black)));
    impl_color!(red, foreground, Color::Ansi16(Standard(Red)));
    impl_color!(green, foreground, Color::Ansi16(Standard(Green)));
    impl_color!(blue, foreground, Color::Ansi16(Standard(Blue)));
    impl_color!(yellow, foreground, Color::Ansi16(Standard(Yellow)));
    impl_color!(magenta, foreground, Color::Ansi16(Standard(Magenta)));
    impl_color!(cyan, foreground, Color::Ansi16(Standard(Cyan)));
    impl_color!(white, foreground, Color::Ansi16(Standard(White)));

    impl_color!(bright_black, foreground, Color::Ansi16(Bright(Black)));
    impl_color!(bright_red, foreground, Color::Ansi16(Bright(Red)));
    impl_color!(bright_green, foreground, Color::Ansi16(Bright(Green)));
    impl_color!(bright_blue, foreground, Color::Ansi16(Bright(Blue)));
    impl_color!(bright_yellow, foreground, Color::Ansi16(Bright(Yellow)));
    impl_color!(bright_magenta, foreground, Color::Ansi16(Bright(Magenta)));
    impl_color!(bright_cyan, foreground, Color::Ansi16(Bright(Cyan)));
    impl_color!(bright_white, foreground, Color::Ansi16(Bright(White)));

    impl_color!(bg_black, background, Color::Ansi16(Standard(Black)));
    impl_color!(bg_red, background, Color::Ansi16(Standard(Red)));
    impl_color!(bg_green, background, Color::Ansi16(Standard(Green)));
    impl_color!(bg_blue, background, Color::Ansi16(Standard(Blue)));
    impl_color!(bg_yellow, background, Color::Ansi16(Standard(Yellow)));
    impl_color!(bg_magenta, background, Color::Ansi16(Standard(Magenta)));
    impl_color!(bg_cyan, background, Color::Ansi16(Standard(Cyan)));
    impl_color!(bg_white, background, Color::Ansi16(Standard(White)));

    impl_color!(bg_bright_black, background, Color::Ansi16(Bright(Black)));
    impl_color!(bg_bright_red, background, Color::Ansi16(Bright(Red)));
    impl_color!(bg_bright_green, background, Color::Ansi16(Bright(Green)));
    impl_color!(bg_bright_blue, background, Color::Ansi16(Bright(Blue)));
    impl_color!(bg_bright_yellow, background, Color::Ansi16(Bright(Yellow)));
    impl_color!(
        bg_bright_magenta,
        background,
        Color::Ansi16(Bright(Magenta))
    );
    impl_color!(bg_bright_cyan, background, Color::Ansi16(Bright(Cyan)));
    impl_color!(bg_bright_white, background, Color::Ansi16(Bright(White)));

    pub fn ansi256_rgb6(mut self, r: u8, g: u8, b: u8) -> Self {
        assert!(r < 6);
        assert!(g < 6);
        assert!(b < 6);
        let code = 16 + r * 36 + g * 6 + b;
        self.foreground = Some(Color::Ansi256(code));
        self
    }

    pub fn ansi256_grayscale24(mut self, lv: u8) -> Self {
        assert!(lv < 24);
        let code = 232 + lv;
        self.foreground = Some(Color::Ansi256(code));
        self
    }

    pub fn true_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground = Some(Color::Rgb(r, g, b));
        self
    }

    pub fn bg_ansi256_rgb6(mut self, r: u8, g: u8, b: u8) -> Self {
        assert!(r < 6);
        assert!(g < 6);
        assert!(b < 6);
        let code = 16 + r * 36 + g * 6 + b;
        self.background = Some(Color::Ansi256(code));
        self
    }

    pub fn bg_ansi256_grayscale24(mut self, lv: u8) -> Self {
        assert!(lv < 24);
        let code = 232 + lv;
        self.background = Some(Color::Ansi256(code));
        self
    }

    pub fn bg_true_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background = Some(Color::Rgb(r, g, b));
        self
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut codes = [0u8; 14]; // max boolean x 4 + color(1 or 3 or 5) x 2 => 14
        let mut n = 0usize;

        if self.bold {
            codes[n] = BOLD;
            n += 1;
        }

        if self.dimmed {
            codes[n] = DIMMED;
            n += 1;
        }

        if self.italic {
            codes[n] = ITALIC;
            n += 1;
        }

        if self.underline {
            codes[n] = UNDERLINE;
            n += 1;
        }

        match self.foreground {
            Some(Color::Ansi16(ref ansi16)) => {
                codes[n] = FOREGROUND + ansi16.code();
                n += 1;
            }
            Some(Color::Ansi256(code)) => {
                codes[n] = 38;
                codes[n + 1] = 5;
                codes[n + 2] = code;
                n += 3;
            }
            Some(Color::Rgb(r, g, b)) => {
                codes[n] = 38;
                codes[n + 1] = 2;
                codes[n + 2] = r;
                codes[n + 3] = g;
                codes[n + 4] = b;
                n += 5;
            }
            None => {}
        }

        match self.background {
            Some(Color::Ansi16(ref ansi16)) => {
                codes[n] = BACKGROUND + ansi16.code();
                n += 1;
            }
            Some(Color::Ansi256(code)) => {
                codes[n] = 48;
                codes[n + 1] = 5;
                codes[n + 2] = code;
                n += 3;
            }
            Some(Color::Rgb(r, g, b)) => {
                codes[n] = 48;
                codes[n + 1] = 2;
                codes[n + 2] = r;
                codes[n + 3] = g;
                codes[n + 4] = b;
                n += 5;
            }
            None => {}
        }

        let iter = &mut codes[0..n].iter();
        if let Some(first) = iter.next() {
            write!(f, "{}{}", CSI, first)?;
            for code in iter {
                write!(f, ";{}", code)?;
            }
            f.write_str("m")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Reset;

impl std::fmt::Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}m", CSI, RESET)
    }
}

pub fn reset() -> Reset {
    Reset
}

#[derive(Debug)]
pub struct Styled<T> {
    target: T,
    style: Style,
}

macro_rules! impl_styled {
    ($fn:ident) => {
        pub fn $fn(mut self) -> Self {
            self.style = self.style.$fn();
            self
        }
    };
}

impl<T> Styled<T> {
    impl_styled!(bold);
    impl_styled!(dimmed);
    impl_styled!(italic);
    impl_styled!(underline);

    impl_styled!(black);
    impl_styled!(red);
    impl_styled!(green);
    impl_styled!(yellow);
    impl_styled!(blue);
    impl_styled!(magenta);
    impl_styled!(cyan);
    impl_styled!(white);

    impl_styled!(bright_black);
    impl_styled!(bright_red);
    impl_styled!(bright_green);
    impl_styled!(bright_yellow);
    impl_styled!(bright_blue);
    impl_styled!(bright_magenta);
    impl_styled!(bright_cyan);
    impl_styled!(bright_white);

    impl_styled!(bg_black);
    impl_styled!(bg_red);
    impl_styled!(bg_green);
    impl_styled!(bg_yellow);
    impl_styled!(bg_blue);
    impl_styled!(bg_magenta);
    impl_styled!(bg_cyan);
    impl_styled!(bg_white);

    impl_styled!(bg_bright_black);
    impl_styled!(bg_bright_red);
    impl_styled!(bg_bright_green);
    impl_styled!(bg_bright_yellow);
    impl_styled!(bg_bright_blue);
    impl_styled!(bg_bright_magenta);
    impl_styled!(bg_bright_cyan);
    impl_styled!(bg_bright_white);

    pub fn ansi256_rgb6(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.ansi256_rgb6(r, g, b);
        self
    }

    pub fn ansi256_grayscale24(mut self, lv: u8) -> Self {
        self.style = self.style.ansi256_grayscale24(lv);
        self
    }

    pub fn true_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.true_color_rgb(r, g, b);
        self
    }

    pub fn bg_ansi256_rgb6(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.bg_ansi256_rgb6(r, g, b);
        self
    }

    pub fn bg_ansi256_grayscale24(mut self, lv: u8) -> Self {
        self.style = self.style.bg_ansi256_grayscale24(lv);
        self
    }

    pub fn bg_true_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.bg_true_color_rgb(r, g, b);
        self
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.style, self.target, Reset)
    }
}

macro_rules! impl_ext_method {
    ($fn:ident) => {
        fn $fn(self) -> Styled<Self> {
            Styled {
                target: self,
                style: Style::default().$fn(),
            }
        }
    };
}

pub trait StyleExt: std::fmt::Display + Sized {
    impl_ext_method!(bold);
    impl_ext_method!(dimmed);
    impl_ext_method!(italic);
    impl_ext_method!(underline);

    impl_ext_method!(black);
    impl_ext_method!(red);
    impl_ext_method!(green);
    impl_ext_method!(yellow);
    impl_ext_method!(blue);
    impl_ext_method!(magenta);
    impl_ext_method!(cyan);
    impl_ext_method!(white);

    impl_ext_method!(bright_black);
    impl_ext_method!(bright_red);
    impl_ext_method!(bright_green);
    impl_ext_method!(bright_yellow);
    impl_ext_method!(bright_blue);
    impl_ext_method!(bright_magenta);
    impl_ext_method!(bright_cyan);
    impl_ext_method!(bright_white);

    impl_ext_method!(bg_black);
    impl_ext_method!(bg_red);
    impl_ext_method!(bg_green);
    impl_ext_method!(bg_yellow);
    impl_ext_method!(bg_blue);
    impl_ext_method!(bg_magenta);
    impl_ext_method!(bg_cyan);
    impl_ext_method!(bg_white);

    impl_ext_method!(bg_bright_black);
    impl_ext_method!(bg_bright_red);
    impl_ext_method!(bg_bright_green);
    impl_ext_method!(bg_bright_yellow);
    impl_ext_method!(bg_bright_blue);
    impl_ext_method!(bg_bright_magenta);
    impl_ext_method!(bg_bright_cyan);
    impl_ext_method!(bg_bright_white);

    fn ansi256_rgb6(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().ansi256_rgb6(r, g, b),
        }
    }

    fn ansi256_grayscale24(self, lv: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().ansi256_grayscale24(lv),
        }
    }

    fn true_color_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().true_color_rgb(r, g, b),
        }
    }

    fn bg_ansi256_rgb6(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().bg_ansi256_rgb6(r, g, b),
        }
    }

    fn bg_ansi256_grayscale24(self, lv: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().bg_ansi256_grayscale24(lv),
        }
    }

    fn bg_true_color_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        Styled {
            target: self,
            style: Style::default().bg_true_color_rgb(r, g, b),
        }
    }
}

impl<T> StyleExt for T where T: std::fmt::Display + Sized {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn style_test() {
        assert_eq!(style().red().to_string(), "\x1B[31m");
        assert_eq!(style().red().bold().to_string(), "\x1B[1;31m");
        assert_eq!(style().white().bg_black().to_string(), "\x1B[37;40m");
        assert_eq!(style().ansi256_rgb6(1, 2, 3).to_string(), "\x1B[38;5;67m");
        assert_eq!(style().to_string(), "");
        assert_eq!(reset().to_string(), "\x1B[0m");
    }

    #[test]
    fn style_ext_test() {
        assert_eq!(
            "xxx".black().bg_white().underline().to_string(),
            "\x1B[4;30;47mxxx\x1B[0m",
        );
    }
}
