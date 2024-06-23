use crate::{style, Style};

const BLUE: Style = style().blue();
const BOLD_YELLOW: Style = style().bold().yellow();

#[derive(Debug)]
pub struct Error {
    pub(crate) on: Option<String>,
    pub(crate) source: std::io::Error,
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self { on: None, source }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BOLD_YELLOW}Command Error:{BOLD_YELLOW:#}")?;
        if let Some(ref about) = self.on {
            write!(f, " {}", about)?;
        }
        writeln!(f)?;
        writeln!(f, "{BLUE}╰─▶{BLUE:#} {}", self.source)?;
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
