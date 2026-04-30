use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};

fn get_handle(path: Option<&str>) -> Result<File, String> {
    // get path if it has value else get value from &env::var or create new path
    let path = match path {
        Some(p) => p,
        None => {
            &env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string())
        }
    };
    //Use OpenOption::new() method to open file with path
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&path)
        .map_err(|e| format!("Error opening file {}: {}", path, e)
        )?;

    Ok(file)
}

pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, String> {
    //get file
    let mut file = get_handle(None)?;
    let mut buffer = String::new();
    //read file to buffer
    file.read_to_string(&mut buffer)
        .map_err(|e| format!("Error reading file: {}", e))?;
    //serde from buffer to Rust code
    let tasks: HashMap<String, T> = serde_json::from_str(&buffer)
        .map_err(|e| format!("Error deserializing file: {}", e))?;
    Ok(tasks)
}

pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), String> {
    let mut file = get_handle(None)?;
    let json = serde_json::to_string(tasks)
        .map_err(|e| format!("Error serializing file: {}", e))?;
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Error writing file: {}", e))?;
    Ok(())
}

pub fn save_one<T>(id: &str, task: &T) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>()
        .unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks).expect("Error at save one func");
    Ok(())
}

pub fn delete_one<T>(id: &str) -> Result<(), String>
where T: Serialize + DeserializeOwned + Clone, {
    let mut tasks = get_all::<T>().
        unwrap_or_else(|_| HashMap::new());
    tasks.remove(id);
    save_all(&tasks).expect("Error at delete one func");
    Ok(())
}