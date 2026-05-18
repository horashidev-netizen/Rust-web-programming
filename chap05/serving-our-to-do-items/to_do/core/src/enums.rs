use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {
    pub(crate) fn from_string(status: &String) -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(format!("invalid status: {}", status))
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::DONE => write!(f, "DONE"),
            &Self::PENDING => write!(f, "PENDING")
        }
    }
}