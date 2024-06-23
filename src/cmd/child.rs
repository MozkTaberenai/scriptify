use super::command::Command;
use super::err::Error;

#[derive(Debug)]
pub(crate) struct Child {
    pub std_child: std::process::Child,
    pub command: Command,
}

impl Child {
    pub fn take_stdin(&mut self) -> Option<std::process::ChildStdin> {
        self.std_child.stdin.take()
    }

    pub fn take_stdout(&mut self) -> Option<std::process::ChildStdout> {
        self.std_child.stdout.take()
    }

    pub fn wait(mut self) -> Result<std::process::ExitStatus, Error> {
        self.std_child.wait().map_err(|source| Error {
            on: Some(self.command.to_string()),
            source,
        })
    }
}
