use crate::enums::TaskStatus;

#[derive(Debug)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub struct Done {
    pub super_struct: Base,
}

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        Done {super_struct: base}
    }
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        Pending {super_struct: base}
    }
}