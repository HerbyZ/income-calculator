use super::CommandResult;
use crate::drawers::global::GlobalDrawer;
use crate::models::{Action, Order, Position};
use crate::utils::console::{ask_confirmation, ask_for_input, wait_for_enter, ConfirmationStatus};
use crate::{exit_with_error, storage};

pub struct GlobalHandler {
    pub drawer: GlobalDrawer,
    positions: Vec<Position>,
}

impl GlobalHandler {
    pub fn new(initial_positions: Vec<Position>) -> GlobalHandler {
        GlobalHandler {
            positions: initial_positions.clone(),
            drawer: GlobalDrawer::new(initial_positions.clone()),
        }
    }

    pub fn handle_command(&mut self, command: String) -> CommandResult {
        match command.trim() {
            "q" => std::process::exit(0),
            "n" => self.handle_next_page(),
            "p" => self.handle_previous_page(),
            "a" => self.handle_add_position(),
            "d" => self.handle_delete_position(),
            "e" => self.handle_edit_position(),
            "h" => self.handle_help(),
            _ => {
                self.drawer.render_positions_table();
                CommandResult::CommandNotFound
            }
        }
    }

    pub fn handle_add_position(&mut self) -> CommandResult {
        let name = match ask_for_input::<String>("Enter position name") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let order_type = match ask_for_input::<String>("Enter order type (long/short)") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let order_type = match order_type.to_lowercase().as_str() {
            "l" | "long" => Action::Long,
            "s" | "short" => Action::Short,
            _ => {
                return CommandResult::Error(format!(
                    "'{}' is not valid position type (long/short)",
                    order_type
                ))
            }
        };

        let amount = match ask_for_input::<f64>("Enter position amount") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };
        let value = match ask_for_input::<f64>("Enter position value") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let id = if let Some(last_position) = self.positions.last() {
            last_position.id + 1
        } else {
            0
        };

        let first_order = Order {
            id: 0,
            action: order_type,
            amount,
            value,
            price: value / amount,
            income: 0f64,
        };

        self.positions
            .push(Position::new(id, name, vec![first_order]));

        self.drawer.positions = self.positions.clone();

        if let Err(error) = storage::save_positions(self.positions.clone()) {
            exit_with_error(error);
        }

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

    pub fn handle_delete_position(&mut self) -> CommandResult {
        let id = match ask_for_input::<i32>("Enter position id") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        if let Err(error) = self.drawer.draw_single_position(id) {
            return CommandResult::Error(error);
        }

        let confirmation = match ask_confirmation(
            format!("Are you sure want to delete position {}? (y,N)", id).as_str(),
            ConfirmationStatus::Rejected,
        ) {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        if confirmation == ConfirmationStatus::Rejected {
            return CommandResult::Ok;
        }

        let pos_index_option = self.positions.iter().position(|pos| pos.id == id);
        if pos_index_option.is_none() {
            return CommandResult::Error(format!("Position with id {} not found", id));
        }

        let pos_index = pos_index_option.unwrap();

        let mut new_positions = self.positions.clone();
        new_positions.remove(pos_index);

        if let Err(error) = self.update_positions(new_positions) {
            exit_with_error(error);
        }

        CommandResult::Ok
    }

    pub fn handle_edit_position(&self) -> CommandResult {
        let id = match ask_for_input::<i32>("Enter position id") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let position = match self.positions.iter().find(|pos| pos.id == id) {
            Some(pos) => pos.to_owned(),
            None => return CommandResult::Error(format!("Position with id '{}' not found", id)),
        };

        CommandResult::ChangeEditMode(super::EditMode::Position(position))
    }

    pub fn handle_help(&self) -> CommandResult {
        self.drawer.draw_help_page();
        if let Err(error) = wait_for_enter() {
            return CommandResult::Error(error);
        }

        CommandResult::Ok
    }

    fn update_positions(&mut self, positions: Vec<Position>) -> Result<(), String> {
        self.positions = positions.clone();
        self.drawer.positions = positions.clone();
        storage::save_positions(positions)
    }
}
