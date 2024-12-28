use crate::storage::Position;

use super::CommandResult;

pub struct PositionHandler {
    position: Option<Position>,
}

impl PositionHandler {
    pub fn new() -> PositionHandler {
        PositionHandler { position: None }
    }

    pub fn handle_command(&self, command: String) -> CommandResult {
        match command.trim() {
            "q" => CommandResult::ChangeEditMode(super::EditMode::Global),
            _ => CommandResult::CommandNotFound,
            // TODO: Add position handler commands
        }
    }

    pub fn change_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}
