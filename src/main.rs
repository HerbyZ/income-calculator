pub mod drawer;
pub mod storage;
pub mod utils;

use drawer::Drawer;
use storage::Position;

pub struct CommandHandler {
    positions: Vec<Position>,
    pub drawer: Drawer,
}

impl CommandHandler {
    fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            positions: initial_positions.clone(),
            drawer: Drawer::new(initial_positions.clone()),
        }
    }

    pub fn handle_add_position(&mut self) -> Result<(), String> {
        let mut name_input = String::new();
        let mut amount_input = String::new();
        let mut value_input = String::new();

        let stdin = std::io::stdin();

        println!("Enter position name:");
        stdin.read_line(&mut name_input).expect("read name");

        println!("Enter position amount:");
        stdin.read_line(&mut amount_input).expect("read amount");

        println!("Enter position value:");
        stdin.read_line(&mut value_input).expect("read value");

        let name = name_input.trim().to_string();
        let amount = amount_input
            .trim()
            .parse::<f64>()
            .expect("parse amount to f64");
        let value = value_input
            .trim()
            .parse::<f64>()
            .expect("parse value to f64");

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
        let mut id_input = String::new();
        let mut sell_price_input = String::new();

        let stdin = std::io::stdin();

        println!("Enter position id:");
        stdin.read_line(&mut id_input).expect("read id input");

        println!("Enter sell price:");
        stdin
            .read_line(&mut sell_price_input)
            .expect("read sell price input");

        let id = id_input.trim().parse::<i32>().expect("parse id to i32");
        let sell_price = sell_price_input
            .trim()
            .parse::<f64>()
            .expect("parse sell price to f64");

        let pos_index_option = self.positions.iter().position(|pos| pos.id == id);
        if pos_index_option.is_none() {
            return Err(format!("Position with id {} not found", id));
        }

        let pos_index = pos_index_option.unwrap();

        let mut position = self.positions[pos_index].clone();
        position.sell_price = sell_price;
        position.income = (sell_price - position.buy_price) * position.amount;

        self.positions[pos_index] = position;
        self.drawer.positions = self.positions.clone();

        if let Err(error) = storage::save_positions(self.positions.clone()) {
            exit_with_error(error);
        }

        Ok(())
    }
}

fn exit_with_error(error: String) {
    println!("ERROR: {}", error);
    std::process::exit(1);
}

fn main() {
    if let Err(error) = storage::initialize_storage() {
        return exit_with_error(error);
    }

    let initial_positions = match storage::load_positions() {
        Ok(positions) => positions,
        Err(error) => return exit_with_error(error),
    };

    let mut command_handler = CommandHandler::new(initial_positions);
    let stdin = std::io::stdin();

    command_handler.drawer.draw_table();

    loop {
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).expect("read command from stdin");

        let command_result: Result<(), String> = match cmd.trim() {
            "q" => break,
            "n" => command_handler.handle_next_page(),
            "p" => command_handler.handle_previous_page(),
            "a" => command_handler.handle_add_position(),
            "c" => command_handler.handle_close_position(),
            _ => continue,
        };

        if let Some(err) = command_result.err() {
            println!("{}", err);
        }

        command_handler.drawer.draw_table();
    }
}
