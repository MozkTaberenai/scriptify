const CSI: &str = "\x1B[";

mod control;
pub mod style;

pub use control::Control;
pub use style::{style, StyleExt, Styled};
