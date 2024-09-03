use crate::color;
use anstyle::Style;

pub const BLUE: Style = Style::new().fg_color(color::BLUE);
pub const MAGENTA: Style = Style::new().fg_color(color::MAGENTA);
pub const BRIGHT_BLACK: Style = Style::new().fg_color(color::BRIGHT_BLACK);
pub const BRIGHT_BLUE: Style = Style::new().fg_color(color::BRIGHT_BLUE);

pub const BOLD_UNDERLINE: Style = Style::new().bold().underline();

pub const BOLD_CYAN: Style = Style::new().bold().fg_color(color::CYAN);
pub const BOLD_YELLOW: Style = Style::new().bold().fg_color(color::YELLOW);

pub const UNDERLINE: Style = Style::new().underline();
pub const UNDERLINE_BRIGHT_BLUE: Style = Style::new().underline().fg_color(color::BRIGHT_BLUE);

pub const RESET: anstyle::Reset = anstyle::Reset;
