use crate::drawers::position::PositionDrawer;
use crate::models::{Action, Order, Position};
use crate::utils::console::ask_for_input;
use crate::{exit_with_error, storage};

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
            "a" => self.handle_add_order(),
            "n" => self.handle_next_page(),
            "p" => self.handle_previous_page(),
            _ => CommandResult::CommandNotFound,
            // TODO: Add position handler commands
        }
    }

    pub fn handle_add_order(&mut self) -> CommandResult {
        let action_input = match ask_for_input::<String>("Enter order type (long/short)") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let action = match Action::from_string(action_input) {
            Ok(action) => action,
            Err(error) => return CommandResult::Error(error),
        };

        let amount = match ask_for_input::<f64>("Enter position amount") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let value = match ask_for_input::<f64>("Enter position value") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let order = Order::new(self.position.clone(), action, amount, value);
        self.position.add_order(order);

        self.drawer.set_position(self.position.clone());
        if let Err(error) = storage::save_position(self.position.clone()) {
            exit_with_error(error);
        };

        CommandResult::Ok
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
