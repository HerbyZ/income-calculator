use super::update_storage;
use crate::models::Position;

pub fn save_positions(positions: &Vec<Position>) -> Result<(), String> {
    update_storage(|storage| storage.positions = positions.clone())
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
    match super::load_storage() {
        Ok(storage) => Ok(storage.positions),
        Err(error) => return Err(error),
    }
}
