use std::path::Path;

use crate::models::Position;

const STORAGE_FILE_PATH: &str = "./storage.json";

pub fn initialize_storage() -> Result<(), String> {
    if Path::new(STORAGE_FILE_PATH).exists() {
        return Ok(());
    };

    match std::fs::write(STORAGE_FILE_PATH, "[]") {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to write initial storage file")),
    }
}

pub fn save_positions(positions: Vec<Position>) -> Result<(), String> {
    let json_string = match serde_json::to_string(&positions) {
        Ok(json) => json,
        Err(_) => return Err(String::from("Failed to serialize positions to json")),
    };

    match std::fs::write(STORAGE_FILE_PATH, json_string) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to save positions to storage file")),
    }
}

pub fn load_positions() -> Result<Vec<Position>, String> {
    let file_content = match std::fs::read_to_string(STORAGE_FILE_PATH) {
        Ok(content) => content,
        Err(_) => return Err(String::from("Failed to read storage file")),
    };

    match serde_json::from_str::<Vec<Position>>(&file_content) {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("Failed to deserialize positions from json")),
    }
}
