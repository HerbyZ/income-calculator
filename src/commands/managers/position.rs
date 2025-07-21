use super::super::utils::commands::parse_arg_or_get_from_input;
use super::super::ChangeEditMode;
use crate::commands::ui::render;
use crate::constants::ORDERS_PER_PAGE;
use crate::models::{Action, Order, Position};
use crate::utils::console::{ask_confirmation, ask_for_input, wait_for_enter, ConfirmationStatus};
use crate::utils::pagination::get_pages_count;
use crate::{exit_with_error, storage};

use super::super::CommandResult;

#[derive(Clone)]
pub struct PositionCommandManager {
    pub position: Position,
    page: i32,
}

impl PositionCommandManager {
    pub fn new(position: &Position) -> PositionCommandManager {
        PositionCommandManager {
            position: position.to_owned(),
            page: 1,
        }
    }

    pub fn handle_command(&mut self, command: String, arg: Option<&String>) -> CommandResult {
        match command.trim() {
            "q" => CommandResult::ChangeEditMode(ChangeEditMode::PositionChanged(
                self.position.clone(),
            )),
            "a" => self.handle_add_order(),
            "d" => self.handle_delete_order(arg),
            "h" => self.handle_help(),
            "n" => self.handle_next_page(),
            "p" => self.handle_previous_page(),
            _ => CommandResult::CommandNotFound,
        }
    }

    pub fn show_ui(&self) {
        render::render_position_info(&self.position, self.page);
        render::render_help_tooltip();
    }

    fn handle_add_order(&mut self) -> CommandResult {
        let action_input = match ask_for_input::<String>("Enter order type (buy/sell)") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let action = match Action::from_string(action_input) {
            Ok(action) => action,
            Err(error) => return CommandResult::Error(error),
        };

        let amount = match ask_for_input::<f64>(
            "Enter position amount.\nEnter 0 if you're fully closing position.",
        ) {
            Ok(value) => {
                if value == 0f64 {
                    self.position.amount
                } else {
                    value
                }
            }
            Err(error) => return CommandResult::Error(error),
        };

        let value = match ask_for_input::<f64>("Enter position value") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let order = Order::new(self.position.clone(), action, amount, value);
        self.position.add_order(order);

        if let Err(error) = storage::save_position(self.position.clone()) {
            exit_with_error(error);
        };

        CommandResult::Ok
    }

    fn handle_delete_order(&mut self, arg: Option<&String>) -> CommandResult {
        let id = match parse_arg_or_get_from_input::<i32>(arg, "Enter position id") {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        let order = match self.position.orders.iter().find(|order| order.id == id) {
            Some(order) => order.to_owned(),
            None => {
                return CommandResult::Error(format!(
                    "Cannot find order with id {} in position {}",
                    id, self.position.id
                ))
            }
        };

        render::render_single_order(&self.position, &order);

        let confirmation = match ask_confirmation(
            format!("Are you sure want to delete order {}? (y,N)", { id }).as_str(),
            ConfirmationStatus::Confirmed,
        ) {
            Ok(value) => value,
            Err(error) => return CommandResult::Error(error),
        };

        if confirmation == ConfirmationStatus::Rejected {
            return CommandResult::Ok;
        }

        if let Err(error) = self.position.remove_order(order.id) {
            return CommandResult::Error(error);
        }

        if let Err(error) = storage::save_position(self.position.clone()) {
            exit_with_error(error);
        };

        CommandResult::Ok
    }

    fn handle_help(&self) -> CommandResult {
        render::render_edit_position_help_page();
        if let Err(error) = wait_for_enter() {
            return CommandResult::Error(error);
        }

        CommandResult::Ok
    }

    fn handle_next_page(&mut self) -> CommandResult {
        let max_page = get_pages_count(self.position.orders.len(), ORDERS_PER_PAGE);
        if (self.page + 1) as f64 > max_page {
            CommandResult::Error(String::from("Already at last page"))
        } else {
            self.page += 1;
            CommandResult::Ok
        }
    }

    fn handle_previous_page(&mut self) -> CommandResult {
        if self.page == 1 {
            CommandResult::Error(String::from("Already at first page"))
        } else {
            self.page -= 1;
            CommandResult::Ok
        }
    }
}
