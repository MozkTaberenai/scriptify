use crate::color;
use anstyle::Style;

// Currently used styles
pub const MAGENTA: Style = Style::new().fg_color(color::MAGENTA);
pub const BRIGHT_BLACK: Style = Style::new().fg_color(color::BRIGHT_BLACK);
pub const BRIGHT_BLUE: Style = Style::new().fg_color(color::BRIGHT_BLUE);

pub const BOLD_UNDERLINE: Style = Style::new().bold().underline();
pub const BOLD_CYAN: Style = Style::new().fg_color(color::CYAN).bold();
pub const UNDERLINE_BRIGHT_BLUE: Style = Style::new().underline().fg_color(color::BRIGHT_BLUE);

// Additional styles for future use
#[allow(dead_code)]
pub const BLUE: Style = Style::new().fg_color(color::BLUE);
#[allow(dead_code)]
pub const RED: Style = Style::new().fg_color(color::RED);
#[allow(dead_code)]
pub const GREEN: Style = Style::new().fg_color(color::GREEN);
#[allow(dead_code)]
pub const YELLOW: Style = Style::new().fg_color(color::YELLOW);

#[allow(dead_code)]
pub const BOLD_RED: Style = Style::new().bold().fg_color(color::RED);
#[allow(dead_code)]
pub const BOLD_GREEN: Style = Style::new().bold().fg_color(color::GREEN);
#[allow(dead_code)]
pub const BOLD_YELLOW: Style = Style::new().bold().fg_color(color::YELLOW);
#[allow(dead_code)]
pub const BOLD_BLUE: Style = Style::new().bold().fg_color(color::BLUE);

#[allow(dead_code)]
pub const UNDERLINE: Style = Style::new().underline();
#[allow(dead_code)]
pub const UNDERLINE_RED: Style = Style::new().underline().fg_color(color::RED);
#[allow(dead_code)]
pub const UNDERLINE_GREEN: Style = Style::new().underline().fg_color(color::GREEN);

#[allow(dead_code)]
pub const RESET: anstyle::Reset = anstyle::Reset;
