pub mod models;
mod positions;

use models::storage::StorageModel;
use models::{FromModel, ToModel};
pub use positions::{load_positions, save_position, save_positions};

use std::path::Path;

use crate::commands::utils::sorting::SortBy;
use crate::constants::STORAGE_FILE_PATH;
use crate::models::Position;

pub struct Storage {
    pub sort_positions_by: SortBy,
    pub positions: Vec<Position>,
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

    let storage_model = match serde_json::from_str::<StorageModel>(&file_content) {
        Ok(data) => data,
        Err(_) => return Err(String::from("Failed to deserialize storage data")),
    };

    storage_model.to_model()
}

pub fn update_storage<F>(func: F) -> Result<(), String>
where
    F: Fn(&mut Storage) -> (),
{
    let mut storage = match load_storage() {
        Ok(storage) => storage,
        Err(error) => return Err(error),
    };

    func(&mut storage);

    let json_string = match serde_json::to_string(&StorageModel::from_model(storage)) {
        Ok(json) => json,
        Err(_) => return Err(String::from("Failed to serialize positions to json")),
    };

    match std::fs::write(STORAGE_FILE_PATH, json_string) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to save positions to storage file")),
    }
}
