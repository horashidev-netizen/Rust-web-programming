use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Pending,
    Done,
}

impl TaskStatus {

    /// Converts a string to a TaskStatus enum.
    ///
    /// # Arguments
    /// * `status` - A reference to a string that is to be converted to a TaskStatus enum
    ///
    /// # Returns
    /// the constructe TaskStatus enum if the string is valid, otherwise an error message
    pub fn from_string(status: &String)
                       -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::Done),
            "PENDING" => Ok(TaskStatus::Pending),
            _ => Err(format!("Invalid status: {}", status))
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::Pending => write!(f, "PENDING"),
            TaskStatus::Done => write!(f, "DONE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::TaskStatus;

    #[test]
    fn it_works() {
        let pending = TaskStatus::Pending;
        let done = TaskStatus::Done;
        assert_eq!(pending, TaskStatus::Pending);
        assert_eq!(done, TaskStatus::Done);
    }
}