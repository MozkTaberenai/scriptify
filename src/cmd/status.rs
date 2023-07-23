#[derive(Debug)]
pub struct Status(pub(crate) Vec<std::process::ExitStatus>);

impl Status {
    pub fn success(&self) -> bool {
        for status in &self.0 {
            if !status.success() {
                return false;
            }
        }
        true
    }

    pub fn code(&self) -> impl Iterator<Item = i32> + '_ {
        self.0.iter().flat_map(|status| status.code())
    }
}
