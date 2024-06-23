mod child;
mod command;
mod err;
mod handle;
mod io;
mod pipeline;
mod spawn;
mod status;

pub use command::Command;
pub use pipeline::Pipe;
pub use spawn::*;

#[cfg(test)]
mod test;
