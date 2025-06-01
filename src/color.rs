use anstyle::{AnsiColor, Color};

// Currently used colors
pub(crate) const MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::Magenta));
pub(crate) const CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
pub(crate) const BRIGHT_BLACK: Option<Color> = Some(Color::Ansi(AnsiColor::BrightBlack));
pub(crate) const BRIGHT_BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::BrightBlue));

// Additional colors for future use
#[allow(dead_code)]
const BLACK: Option<Color> = Some(Color::Ansi(AnsiColor::Black));
#[allow(dead_code)]
pub(crate) const RED: Option<Color> = Some(Color::Ansi(AnsiColor::Red));
#[allow(dead_code)]
pub(crate) const GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
#[allow(dead_code)]
pub(crate) const YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::Yellow));
#[allow(dead_code)]
pub(crate) const BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
#[allow(dead_code)]
const WHITE: Option<Color> = Some(Color::Ansi(AnsiColor::White));
#[allow(dead_code)]
const BRIGHT_RED: Option<Color> = Some(Color::Ansi(AnsiColor::BrightRed));
#[allow(dead_code)]
const BRIGHT_GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::BrightGreen));
#[allow(dead_code)]
const BRIGHT_YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::BrightYellow));
#[allow(dead_code)]
const BRIGHT_MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::BrightMagenta));
#[allow(dead_code)]
const BRIGHT_CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::BrightCyan));
#[allow(dead_code)]
const BRIGHT_WHITE: Option<Color> = Some(Color::Ansi(AnsiColor::BrightWhite));
