
pub enum TaskStatus {
    DONE,
    PENDING
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            TaskStatus::DONE => write!(f, "{}", "This is DONE"),
            TaskStatus::PENDING => write!(f, "{}", "This is PENDING"),
        }
    }
}