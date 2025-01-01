use crate::drawers::position::PositionDrawer;
use crate::models::Position;

use super::CommandResult;

#[derive(Clone)]
pub struct PositionHandler {
    pub position: Position,
    pub drawer: PositionDrawer,
}

impl PositionHandler {
    pub fn new(position: Position) -> PositionHandler {
        PositionHandler {
            position: position.clone(),
            drawer: PositionDrawer::new(position),
        }
    }

    pub fn handle_command(&mut self, command: String) -> CommandResult {
        match command.trim() {
            "q" => CommandResult::ChangeEditMode(super::EditMode::Global),
            "n" => self.handle_next_page(),
            "p" => self.handle_previous_page(),
            _ => CommandResult::CommandNotFound,
            // TODO: Add position handler commands
        }
    }

    pub fn handle_next_page(&mut self) -> CommandResult {
        match self.drawer.next_page() {
            Ok(()) => CommandResult::Ok,
            Err(error) => CommandResult::Error(error),
        }
    }

    pub fn handle_previous_page(&mut self) -> CommandResult {
        match self.drawer.previous_page() {
            Ok(()) => CommandResult::Ok,
            Err(error) => CommandResult::Error(error),
        }
    }
}
