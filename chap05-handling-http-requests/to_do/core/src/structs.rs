use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::enums::TaskStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.status {
            TaskStatus::Done => write!(f,"Done, {}", self.title),
            TaskStatus::Pending => write!(f,"Pending, {}", self.title),
        }
    }
}

impl AllToDoItems {
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>) -> AllToDoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();
        for (_, item) in all_items {
            match item.status {
                TaskStatus::Done => pending.push(item),
                TaskStatus::Pending => done.push(item),
            }
        }
        AllToDoItems { pending, done }
    }
}