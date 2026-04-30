use crate::enums::TaskStatus;
use crate::structs::base::Base;
pub struct Done {
    pub super_struct: Base
}
impl Done {
    pub fn new(input_string: &str) -> Self {
        let base = Base {
            title: input_string.to_string(),
            status: TaskStatus::DONE
        };
        Done { super_struct: base}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_task() {
        let done_task = Done::new("Test Task");
        assert_eq!(done_task.super_struct.title, "Test Task");
        assert_eq!(done_task.super_struct.status, TaskStatus::DONE);
    }
}