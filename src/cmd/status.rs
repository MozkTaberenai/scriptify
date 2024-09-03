/// Describes the result of a processes after it has terminated.
#[derive(Debug)]
pub struct Status(pub(crate) Vec<std::process::ExitStatus>);

impl Status {
    /// Returns `true` if all processes have terminated successfully.
    pub fn success(&self) -> bool {
        for status in &self.0 {
            if !status.success() {
                return false;
            }
        }
        true
    }

    /// Returns an iterator over the exit codes of all processes.
    pub fn code(&self) -> impl Iterator<Item = i32> + '_ {
        self.0.iter().flat_map(|status| status.code())
    }
}
