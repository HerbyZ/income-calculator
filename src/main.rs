use prettytable::{row, Table};

pub mod utils;

use utils::math::round;

struct Position {
    name: String,
    amount: f64,
    value: f64,
    buy_price: f64,
    sell_price: f64,
    income: f64,
}

fn get_position_info() -> Position {
    let mut name_input = String::new();
    let mut amount_input = String::new();
    let mut value_input = String::new();

    let stdin = std::io::stdin();

    println!("Enter position name:");
    stdin
        .read_line(&mut name_input)
        .expect("Failed to read name");

    println!("Enter position amount:");
    stdin
        .read_line(&mut amount_input)
        .expect("Failed to read amount");

    println!("Enter position value:");
    stdin
        .read_line(&mut value_input)
        .expect("Failed to read amount");

    let name = name_input.trim().to_string();
    let amount = amount_input
        .trim()
        .parse::<f64>()
        .expect("Failed to parse amount to f64");
    let value = value_input
        .trim()
        .parse::<f64>()
        .expect("Failed to parse amount to f64");

    Position {
        name,
        amount,
        value,
        buy_price: value / amount,
        sell_price: 0f64,
        income: 0f64,
    }
}

fn show_positions_table(positions: Vec<Position>) {
    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Amount",
        "Value",
        "Buy price",
        "Sell price",
        "Income"
    ]);

    positions.iter().for_each(|position| {
        table.add_row(row![
            position.name,
            round(position.amount).unwrap(),
            round(position.value).unwrap(),
            round(position.buy_price).unwrap(),
            round(position.sell_price).unwrap(),
            round(position.income).unwrap(),
        ]);
    });

    table.printstd();
}

fn main() {
    let mut positions: Vec<Position> = vec![];

    let position = get_position_info();
    positions.push(position);

    show_positions_table(positions);
}
