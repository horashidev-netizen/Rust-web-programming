#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

use std::fmt::Formatter;
use crate::enums::TaskStatus;
use crate::structs::{
    done::Done,
    pending::Pending
};


pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

impl std::fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ItemTypes::Pending(pending) => write!(f, "Pending: {}", pending.super_struct.title),
            ItemTypes::Done(done) => write!(f, "Done: {}", done.super_struct.title)
        }
    }
}


pub fn create(title: &str, task_status: TaskStatus) -> ItemTypes {
    let _ = save_one(&title.to_string(), &task_status.to_string());
    match task_status {
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(&title)),
        TaskStatus::DONE => ItemTypes::Done(Done::new(&title))
    }
}