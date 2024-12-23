use crate::{
    exit_with_error, storage,
    utils::input::{ask_confirmation, ask_for_input, wait_for_enter, ConfirmationStatus},
    Drawer, Position,
};

pub struct CommandHandler {
    pub drawer: Drawer,
    positions: Vec<Position>,
}

impl CommandHandler {
    pub fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            positions: initial_positions.clone(),
            drawer: Drawer::new(initial_positions.clone()),
        }
    }

    pub fn handle_add_position(&mut self) -> Result<(), String> {
        let name = match ask_for_input::<String>("Enter position name") {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let amount = match ask_for_input::<f64>("Enter position amount") {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let value = match ask_for_input::<f64>("Enter position value") {
            Ok(value) => value,
            Err(error) => return Err(error),
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

        Ok(())
    }

    pub fn handle_next_page(&mut self) -> Result<(), String> {
        self.drawer.next_page()
    }

    pub fn handle_previous_page(&mut self) -> Result<(), String> {
        self.drawer.previous_page()
    }

    pub fn handle_close_position(&mut self) -> Result<(), String> {
        let id = match ask_for_input::<i32>("Enter position id") {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let sell_price = match ask_for_input::<f64>("Enter sell price") {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let pos_index_option = self.positions.iter().position(|pos| pos.id == id);
        if pos_index_option.is_none() {
            return Err(format!("Position with id {} not found", id));
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

        Ok(())
    }

    pub fn handle_delete_position(&mut self) -> Result<(), String> {
        let id = match ask_for_input::<i32>("Enter position id") {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        if let Err(error) = self.drawer.draw_single_position(id) {
            return Err(error);
        }

        let confirmation = match ask_confirmation(
            format!("Are you sure want to delete position {}? (y,N)", id).as_str(),
            ConfirmationStatus::Rejected,
        ) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        if confirmation == ConfirmationStatus::Rejected {
            return Ok(());
        }

        let pos_index_option = self.positions.iter().position(|pos| pos.id == id);
        if pos_index_option.is_none() {
            return Err(format!("Position with id {} not found", id));
        }

        let pos_index = pos_index_option.unwrap();

        let mut new_positions = self.positions.clone();
        new_positions.remove(pos_index);

        if let Err(error) = self.update_positions(new_positions) {
            exit_with_error(error);
        }

        Ok(())
    }

    pub fn handle_help(&self) -> Result<(), String> {
        self.drawer.draw_help_page();
        wait_for_enter()
    }

    fn update_positions(&mut self, positions: Vec<Position>) -> Result<(), String> {
        self.positions = positions.clone();
        self.drawer.positions = positions.clone();
        storage::save_positions(positions)
    }
}
