/// Shorthand function of Style::new()
pub const fn style() -> Style {
    Style::new()
}

/// ANSI Text styling
#[derive(Debug, Default, Clone, Copy)]
pub struct Style(anstyle::Style);

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Style {
    pub const fn new() -> Self {
        Self(anstyle::Style::new())
    }

    pub const fn blink(self) -> Self {
        Self(self.0.blink())
    }

    pub const fn bold(self) -> Self {
        Self(self.0.bold())
    }

    pub const fn dimmed(self) -> Self {
        Self(self.0.dimmed())
    }

    pub const fn hidden(self) -> Self {
        Self(self.0.hidden())
    }

    pub const fn invert(self) -> Self {
        Self(self.0.invert())
    }

    pub const fn italic(self) -> Self {
        Self(self.0.italic())
    }

    pub const fn strikethrough(self) -> Self {
        Self(self.0.strikethrough())
    }

    pub const fn underline(self) -> Self {
        Self(self.0.underline())
    }

    pub const fn reset(self) -> Self {
        Self::new()
    }
}

macro_rules! impl_color {
    ($color:ident, $ground:ident, $ansi:ident) => {
        impl Style {
            pub const fn $color(self) -> Self {
                Self(
                    self.0
                        .$ground(Some(anstyle::Color::Ansi(anstyle::AnsiColor::$ansi))),
                )
            }
        }
    };
}

impl_color!(black, fg_color, Black);
impl_color!(red, fg_color, Red);
impl_color!(green, fg_color, Green);
impl_color!(yellow, fg_color, Yellow);
impl_color!(blue, fg_color, Blue);
impl_color!(magenta, fg_color, Magenta);
impl_color!(cyan, fg_color, Cyan);
impl_color!(white, fg_color, White);
impl_color!(bright_black, fg_color, BrightBlack);
impl_color!(bright_red, fg_color, BrightRed);
impl_color!(bright_green, fg_color, BrightGreen);
impl_color!(bright_yellow, fg_color, BrightYellow);
impl_color!(bright_blue, fg_color, BrightBlue);
impl_color!(bright_magenta, fg_color, BrightMagenta);
impl_color!(bright_cyan, fg_color, BrightCyan);
impl_color!(bright_white, fg_color, BrightWhite);

impl_color!(bg_black, bg_color, Black);
impl_color!(bg_red, bg_color, Red);
impl_color!(bg_green, bg_color, Green);
impl_color!(bg_yellow, bg_color, Yellow);
impl_color!(bg_blue, bg_color, Blue);
impl_color!(bg_magenta, bg_color, Magenta);
impl_color!(bg_cyan, bg_color, Cyan);
impl_color!(bg_white, bg_color, White);
impl_color!(bg_bright_black, bg_color, BrightBlack);
impl_color!(bg_bright_red, bg_color, BrightRed);
impl_color!(bg_bright_green, bg_color, BrightGreen);
impl_color!(bg_bright_yellow, bg_color, BrightYellow);
impl_color!(bg_bright_blue, bg_color, BrightBlue);
impl_color!(bg_bright_magenta, bg_color, BrightMagenta);
impl_color!(bg_bright_cyan, bg_color, BrightCyan);
impl_color!(bg_bright_white, bg_color, BrightWhite);

impl_color!(underline_black, underline_color, Black);
impl_color!(underline_red, underline_color, Red);
impl_color!(underline_green, underline_color, Green);
impl_color!(underline_yellow, underline_color, Yellow);
impl_color!(underline_blue, underline_color, Blue);
impl_color!(underline_magenta, underline_color, Magenta);
impl_color!(underline_cyan, underline_color, Cyan);
impl_color!(underline_white, underline_color, White);
impl_color!(underline_bright_black, underline_color, BrightBlack);
impl_color!(underline_bright_red, underline_color, BrightRed);
impl_color!(underline_bright_green, underline_color, BrightGreen);
impl_color!(underline_bright_yellow, underline_color, BrightYellow);
impl_color!(underline_bright_blue, underline_color, BrightBlue);
impl_color!(underline_bright_magenta, underline_color, BrightMagenta);
impl_color!(underline_bright_cyan, underline_color, BrightCyan);
impl_color!(underline_bright_white, underline_color, BrightWhite);
