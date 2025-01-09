mod models;

use std::path::Path;

use models::{FromModel, PositionStorageModel, ToModel};

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
    let mut position_models = vec![];
    for pos in positions {
        position_models.push(PositionStorageModel::from_model(pos))
    }

    let json_string = match serde_json::to_string(&position_models) {
        Ok(json) => json,
        Err(_) => return Err(String::from("Failed to serialize positions to json")),
    };

    match std::fs::write(STORAGE_FILE_PATH, json_string) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to save positions to storage file")),
    }
}

pub fn save_position(position: Position) -> Result<(), String> {
    let mut positions = match load_positions() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let pos_index = match positions
        .iter()
        .position(|pos_candidate| pos_candidate.id == position.id)
    {
        Some(index) => index,
        None => return Err(format!("Position {} not found", position.id)),
    };

    positions[pos_index] = position;

    save_positions(positions)
}

pub fn load_positions() -> Result<Vec<Position>, String> {
    let file_content = match std::fs::read_to_string(STORAGE_FILE_PATH) {
        Ok(content) => content,
        Err(_) => return Err(String::from("Failed to read storage file")),
    };

    let position_models = match serde_json::from_str::<Vec<PositionStorageModel>>(&file_content) {
        Ok(data) => data,
        Err(_) => return Err(String::from("Failed to deserialize positions from json")),
    };

    let mut positions = vec![];
    for storage_model in position_models {
        positions.push(match storage_model.to_model() {
            Ok(model) => model,
            Err(error) => return Err(error),
        });
    }

    Ok(positions)
}
