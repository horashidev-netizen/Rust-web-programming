use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::de::{DeserializeOwned};
use serde::Serialize;

pub fn get_handle(path: Option<&str>) -> Result<File, String> {
    let path =path.unwrap_or("./task.json");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .map_err(|err| {
            format!("Error opening file {}: {:?}", path, err)
        })?;
        Ok(file)
}

pub fn get_all<T: DeserializeOwned + Debug>() -> Result<HashMap<String,T>, String> {
    let mut file = get_handle(None)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e|
        format!("Error reading file: {}", e)
    )?;
    if contents.trim().is_empty() {
        return Ok(HashMap::new());
    }
    let tasks = serde_json::from_str(&contents).map_err(|err| {
        format!("Error parsing file: {}", err)
    })?;
    println!("{:?}", tasks);
    Ok(tasks)
}

pub fn save_all<T: Serialize>(items: &HashMap<String,T>) -> Result<(),String> {
    let mut file = get_handle(None)?;
    let json = serde_json::to_string(items).map_err(|e| {
        format!("Error serializing file: {}", e)
    })?;
    file.write_all(json.as_bytes()).map_err(|e| {
        format!("Error writing file at save all func: {}", e)
    })?;

    Ok(())
}

pub fn save_one<T: Serialize + Clone + DeserializeOwned + Debug>(id: &str, task: &T) -> Result<(), String> {
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}

pub fn get_one<T:Clone + DeserializeOwned + Debug>(id: &str) -> Result<T, String> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err("No such task".to_string()),
    }

}

pub fn delete_one<T:Serialize + DeserializeOwned + Debug>(id: &str) -> Result<(), String> {
    let mut tasks = get_all::<T>()?;
    if tasks.remove(id).is_some() {
        save_all(&tasks)?;
        Ok(())
    } else {
        Err(format!("Task {} not found", id))
    }
}