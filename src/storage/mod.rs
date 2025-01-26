mod models;
mod positions;

pub use positions::{load_positions, save_position, save_positions};

use models::PositionStorageModel;
use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::constants::STORAGE_FILE_PATH;

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    positions: Vec<PositionStorageModel>,
}

const DEFAULT_STORAGE_FILE_CONTENT: &str = "{ positions: [] }";

pub fn initialize_storage() -> Result<(), String> {
    if Path::new(STORAGE_FILE_PATH).exists() {
        return Ok(());
    };

    match std::fs::write(STORAGE_FILE_PATH, DEFAULT_STORAGE_FILE_CONTENT) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to write initial storage file")),
    }
}

pub fn reinitialize_storage() -> Result<(), String> {
    match std::fs::write(STORAGE_FILE_PATH, DEFAULT_STORAGE_FILE_CONTENT) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to write initial storage file")),
    }
}

pub fn load_storage() -> Result<Storage, String> {
    let file_content = match std::fs::read_to_string(STORAGE_FILE_PATH) {
        Ok(content) => content,
        Err(_) => return Err(String::from("Failed to read storage file")),
    };

    match serde_json::from_str::<Storage>(&file_content) {
        Ok(data) => Ok(data),
        Err(_) => return Err(String::from("Failed to deserialize storage data")),
    }
}
