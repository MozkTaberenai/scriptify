use smallvec::SmallVec;

const CSI: &str = "\x1B[";

#[derive(Debug)]
pub enum Control {
    CursorUp(u16),      // xA
    CursorDown(u16),    // xB
    CursorForward(u16), // xC
    CursorBack(u16),    // xD

    CursorNextLine(u16),     // xE
    CursorPreviousLine(u16), // xF

    CursorMoveInLine(u16), // xG
    CursorMove(u16, u16),  // x;yH

    EraseForward, // 0J
    EraseBack,    // 1J
    EraseDisplay, // 2J
    EraseAll,     // 3J

    EraseForwardInLine, // 0K
    EraseBackInLine,    // 1K
    EraseLine,          // 2K

    SaveCursor,    // s
    RestoreCursor, // u

    ShowCursor, // ?25h
    HideCursor, // ?25l
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Control::CursorUp(n) => write!(f, "{CSI}{n}A"),
            Control::CursorDown(n) => write!(f, "{CSI}{n}B"),
            Control::CursorForward(n) => write!(f, "{CSI}{n}C"),
            Control::CursorBack(n) => write!(f, "{CSI}{n}D"),

            Control::CursorNextLine(n) => write!(f, "{CSI}{n}E"),
            Control::CursorPreviousLine(n) => write!(f, "{CSI}{n}F"),

            Control::CursorMoveInLine(n) => write!(f, "{CSI}{n}G"),
            Control::CursorMove(x, y) => write!(f, "{CSI}{x};{y}H"),

            Control::EraseForward => write!(f, "{CSI}0J"),
            Control::EraseBack => write!(f, "{CSI}1J"),
            Control::EraseDisplay => write!(f, "{CSI}2J"),
            Control::EraseAll => write!(f, "{CSI}3J"),

            Control::EraseForwardInLine => write!(f, "{CSI}0K"),
            Control::EraseBackInLine => write!(f, "{CSI}1K"),
            Control::EraseLine => write!(f, "{CSI}2K"),

            Control::SaveCursor => write!(f, "{CSI}s"),
            Control::RestoreCursor => write!(f, "{CSI}u"),

            Control::ShowCursor => write!(f, "{CSI}?25h"),
            Control::HideCursor => write!(f, "{CSI}?25l"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Style(SmallVec<[u8; 8]>);

macro_rules! impl_style_method {
    ($fn:ident, $code:expr) => {
        pub fn $fn(mut self) -> Self {
            self.0.push($code);
            self
        }
    };
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    impl_style_method!(reset, 0);

    impl_style_method!(bold, 1);
    impl_style_method!(dimmed, 2);
    impl_style_method!(italic, 3);
    impl_style_method!(underline, 4);

    impl_style_method!(black, 30);
    impl_style_method!(red, 31);
    impl_style_method!(green, 32);
    impl_style_method!(yellow, 33);
    impl_style_method!(blue, 34);
    impl_style_method!(magenta, 35);
    impl_style_method!(cyan, 36);
    impl_style_method!(white, 37);

    pub fn color_8bit(mut self, n: u8) -> Self {
        self.0.push(38);
        self.0.push(5);
        self.0.push(n);
        self
    }

    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.0.push(38);
        self.0.push(2);
        self.0.push(r);
        self.0.push(g);
        self.0.push(b);
        self
    }

    impl_style_method!(default_color, 39);

    impl_style_method!(bg_black, 40);
    impl_style_method!(bg_red, 41);
    impl_style_method!(bg_green, 42);
    impl_style_method!(bg_yellow, 43);
    impl_style_method!(bg_blue, 44);
    impl_style_method!(bg_magenta, 45);
    impl_style_method!(bg_cyan, 46);
    impl_style_method!(bg_white, 47);

    pub fn bg_color_8bit(mut self, n: u8) -> Self {
        self.0.push(48);
        self.0.push(5);
        self.0.push(n);
        self
    }

    pub fn bg_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.0.push(48);
        self.0.push(2);
        self.0.push(r);
        self.0.push(g);
        self.0.push(b);
        self
    }

    impl_style_method!(bg_default_color, 49);

    impl_style_method!(bright_black, 90);
    impl_style_method!(bright_red, 91);
    impl_style_method!(bright_green, 92);
    impl_style_method!(bright_yellow, 93);
    impl_style_method!(bright_blue, 94);
    impl_style_method!(bright_magenta, 95);
    impl_style_method!(bright_cyan, 96);
    impl_style_method!(bright_white, 97);

    impl_style_method!(bg_bright_black, 100);
    impl_style_method!(bg_bright_red, 101);
    impl_style_method!(bg_bright_green, 102);
    impl_style_method!(bg_bright_yellow, 103);
    impl_style_method!(bg_bright_blue, 104);
    impl_style_method!(bg_bright_magenta, 105);
    impl_style_method!(bg_bright_cyan, 106);
    impl_style_method!(bg_bright_white, 107);
}

pub fn style() -> Style {
    Style::default()
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(code) = iter.next() {
            write!(f, "{CSI}{code}")?;
            for code in iter {
                write!(f, ";{code}")?;
            }
            write!(f, "m")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Styled<T> {
    target: T,
    style: Style,
}

macro_rules! impl_styled_method {
    ($fn:ident) => {
        pub fn $fn(mut self) -> Self {
            self.style = self.style.$fn();
            self
        }
    };
}

impl<T> Styled<T> {
    impl_styled_method!(bold);
    impl_styled_method!(dimmed);
    impl_styled_method!(italic);
    impl_styled_method!(underline);

    impl_styled_method!(black);
    impl_styled_method!(red);
    impl_styled_method!(green);
    impl_styled_method!(yellow);
    impl_styled_method!(blue);
    impl_styled_method!(magenta);
    impl_styled_method!(cyan);
    impl_styled_method!(white);

    pub fn color_8bit(mut self, n: u8) -> Self {
        self.style = self.style.color_8bit(n);
        self
    }

    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.color_rgb(r, g, b);
        self
    }

    impl_styled_method!(default_color);

    impl_styled_method!(bg_black);
    impl_styled_method!(bg_red);
    impl_styled_method!(bg_green);
    impl_styled_method!(bg_yellow);
    impl_styled_method!(bg_blue);
    impl_styled_method!(bg_magenta);
    impl_styled_method!(bg_cyan);
    impl_styled_method!(bg_white);

    pub fn bg_color_8bit(mut self, n: u8) -> Self {
        self.style = self.style.bg_color_8bit(n);
        self
    }

    pub fn bg_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.bg_color_rgb(r, g, b);
        self
    }

    impl_styled_method!(bg_default_color);

    impl_styled_method!(bright_black);
    impl_styled_method!(bright_red);
    impl_styled_method!(bright_green);
    impl_styled_method!(bright_yellow);
    impl_styled_method!(bright_blue);
    impl_styled_method!(bright_magenta);
    impl_styled_method!(bright_cyan);
    impl_styled_method!(bright_white);

    impl_styled_method!(bg_bright_black);
    impl_styled_method!(bg_bright_red);
    impl_styled_method!(bg_bright_green);
    impl_styled_method!(bg_bright_yellow);
    impl_styled_method!(bg_bright_blue);
    impl_styled_method!(bg_bright_magenta);
    impl_styled_method!(bg_bright_cyan);
    impl_styled_method!(bg_bright_white);
}

impl<T: std::fmt::Display> std::fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.style, self.target, style().reset())
    }
}

macro_rules! impl_ext_method {
    ($fn:ident) => {
        fn $fn(self) -> Styled<Self> {
            let style = style().$fn();
            Styled {
                target: self,
                style,
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

    fn color_8bit(self, n: u8) -> Styled<Self> {
        let style = style().color_8bit(n);
        Styled {
            target: self,
            style,
        }
    }

    fn color_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        let style = style().color_rgb(r, g, b);
        Styled {
            target: self,
            style,
        }
    }

    impl_ext_method!(bg_default_color);

    impl_ext_method!(bg_black);
    impl_ext_method!(bg_red);
    impl_ext_method!(bg_green);
    impl_ext_method!(bg_yellow);
    impl_ext_method!(bg_blue);
    impl_ext_method!(bg_magenta);
    impl_ext_method!(bg_cyan);
    impl_ext_method!(bg_white);

    fn bg_color_8bit(self, n: u8) -> Styled<Self> {
        let style = style().bg_color_8bit(n);
        Styled {
            target: self,
            style,
        }
    }

    fn bg_color_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        let style = style().bg_color_rgb(r, g, b);
        Styled {
            target: self,
            style,
        }
    }

    impl_ext_method!(default_color);

    impl_ext_method!(bright_black);
    impl_ext_method!(bright_red);
    impl_ext_method!(bright_green);
    impl_ext_method!(bright_yellow);
    impl_ext_method!(bright_blue);
    impl_ext_method!(bright_magenta);
    impl_ext_method!(bright_cyan);
    impl_ext_method!(bright_white);

    impl_ext_method!(bg_bright_black);
    impl_ext_method!(bg_bright_red);
    impl_ext_method!(bg_bright_green);
    impl_ext_method!(bg_bright_yellow);
    impl_ext_method!(bg_bright_blue);
    impl_ext_method!(bg_bright_magenta);
    impl_ext_method!(bg_bright_cyan);
    impl_ext_method!(bg_bright_white);
}

impl<T> StyleExt for T where T: std::fmt::Display + Sized {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn control_test() {
        assert_eq!(Control::CursorUp(3).to_string(), "\x1B[3A");
    }

    #[test]
    fn style_test() {
        assert_eq!(style().red().to_string(), "\x1B[31m");
        assert_eq!(style().red().bold().to_string(), "\x1B[31;1m");
        assert_eq!(style().to_string(), "");
        assert_eq!(style().reset().to_string(), "\x1B[0m");
    }

    #[test]
    fn style_ext_test() {
        assert_eq!(
            "xxx".black().bg_white().underline().to_string(),
            "\x1B[30;47;4mxxx\x1B[0m",
        );
    }
}
