use super::load_storage;
use super::models::{FromModel, PositionStorageModel, ToModel};
use crate::constants::STORAGE_FILE_PATH;
use crate::models::Position;

pub fn save_positions(positions: &Vec<Position>) -> Result<(), String> {
    let mut position_models = vec![];
    for pos in positions.to_vec() {
        position_models.push(PositionStorageModel::from_model(pos))
    }

    let mut storage = match load_storage() {
        Ok(storage) => storage,
        Err(error) => return Err(error),
    };
    storage.positions = position_models;

    let json_string = match serde_json::to_string(&storage) {
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

    save_positions(&positions)
}

pub fn load_positions() -> Result<Vec<Position>, String> {
    let storage = match super::load_storage() {
        Ok(storage) => storage,
        Err(error) => return Err(error),
    };

    let mut positions = vec![];
    for storage_model in storage.positions {
        positions.push(match storage_model.to_model() {
            Ok(model) => model,
            Err(error) => return Err(error),
        });
    }

    Ok(positions)
}
