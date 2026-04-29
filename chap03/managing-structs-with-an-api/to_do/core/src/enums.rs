use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    PENDING,
    DONE
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "PENDING" => Ok(TaskStatus::PENDING),
            "DONE" => Ok(TaskStatus::DONE),
            _ => Err(format!("Unknown task status: {}", status))
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::PENDING => f.write_str("This process is PENDING"),
            &Self::DONE => f.write_str("This process is DONE")
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_status() {
        assert_eq!(TaskStatus::PENDING.to_string(), "This process is PENDING");
        assert_eq!(TaskStatus::DONE.to_string(), "This process is DONE");
        let done_status = format!("{}", TaskStatus::DONE);
        let pending_status = format!("{}", TaskStatus::PENDING);

        assert_eq!(done_status, "This process is DONE");
        assert_eq!(pending_status, "This process is PENDING");

        let done_status = TaskStatus::DONE.to_string();
        let pending_status = TaskStatus::PENDING.to_string();

        assert_eq!(done_status, "This process is DONE");
        assert_eq!(pending_status, "This process is PENDING");
    }
}