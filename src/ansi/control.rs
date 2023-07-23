use super::CSI;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Control::CursorUp(3).to_string(), "\x1B[3A");
        assert_eq!(Control::CursorDown(10).to_string(), "\x1B[10B");
    }
}
