#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExitCode {
    Success,
    Failure,
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        match self {
            ExitCode::Success => 0,
            ExitCode::Failure => 1,
        }
    }
}
