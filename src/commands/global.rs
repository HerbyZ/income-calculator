use super::CommandResult;
use crate::drawers::global::GlobalDrawer;
use crate::utils::input::{ask_confirmation, ask_for_input, wait_for_enter, ConfirmationStatus};
use crate::{exit_with_error, storage, Position};

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
            "c" => self.handle_close_position(),
            "d" => self.handle_delete_position(),
            "h" => self.handle_help(),
            _ => {
                self.drawer.draw_table();
                CommandResult::CommandNotFound
            }
        }
    }

    pub fn handle_add_position(&mut self) -> CommandResult {
        let name = match ask_for_input::<String>("Enter position name") {
            Ok(value) => value,
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

        let id = if let Some(last_position) = self.positions.last() {
            last_position.id + 1
        } else {
            0
        };

        self.positions.push(Position {
            id,
            name,
            amount,
            value,
            buy_price: value / amount,
            sell_price: 0f64,
            income: 0f64,
        });
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

    pub fn handle_close_position(&mut self) -> CommandResult {
        let id = match ask_for_input::<i32>("Enter position id") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let sell_price = match ask_for_input::<f64>("Enter sell price") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let pos_index_option = self.positions.iter().position(|pos| pos.id == id);
        if pos_index_option.is_none() {
            return CommandResult::Error(format!("Position with id {} not found", id));
        }

        let pos_index = pos_index_option.unwrap();

        let mut position = self.positions[pos_index].clone();
        position.sell_price = sell_price;
        position.income = (sell_price - position.buy_price) * position.amount;

        let mut new_positions = self.positions.clone();
        new_positions[pos_index] = position;

        if let Err(error) = self.update_positions(new_positions) {
            exit_with_error(error);
        }

        CommandResult::Ok
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
