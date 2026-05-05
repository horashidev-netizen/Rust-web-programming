use crate::structs::ToDoItem;
use crate::enums::TaskStatus;
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;


pub fn create(title: &str, status: TaskStatus) -> Result<ToDoItem, String> {
    let item = ToDoItem {
        title: title.to_string(),
        status,
    };
    let _ = save_one(title, &item)?;
    Ok(item)
}