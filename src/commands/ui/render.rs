use colored::Colorize;
use prettytable::{row, Table};

use crate::constants::{ORDERS_PER_PAGE, POISITIONS_PER_PAGE};
use crate::models::{Action, Order, Position};
use crate::utils::console::clear_screen;
use crate::utils::math::round;
use crate::utils::pagination::{draw_page_counter, get_pages_count, select_items_for_page};

pub fn render_positions_table(positions: &Vec<Position>, page: i32) {
    clear_screen().expect("clean screen");
    let mut table = Table::new();
    table.add_row(row![
        "Id",
        "Name",
        "Amount",
        "Avg value",
        "Avg price",
        "Income"
    ]);

    let mut reversed_positions = positions.to_vec();
    reversed_positions.reverse();

    let positions_to_draw = select_items_for_page(reversed_positions, page, POISITIONS_PER_PAGE);

    positions_to_draw.iter().for_each(|position| {
        table.add_row(row![
            position.id,
            position.name,
            round(position.amount).unwrap(),
            round(position.avg_value).unwrap(),
            round(position.avg_price).unwrap(),
            round(position.income).unwrap(),
        ]);
    });

    // Add total row
    let total = calculate_total(positions);
    table.add_row(row![
        "Total",
        "-",
        "-",
        round(total.0).unwrap(),
        "-",
        round(total.1).unwrap()
    ]);

    table.printstd();
    draw_page_counter(page, get_pages_count(positions.len(), POISITIONS_PER_PAGE));
}

pub fn render_help_tooltip() {
    println!("{}", "Type 'h' for help".italic().bright_black());
}

pub fn render_global_help_page() {
    clear_screen().expect("clear screen");
    println!("{}\n", "Available commands:".bold());
    println!("{} - {}", "h".bold().yellow(), "Show help page");
    println!("{} - {}", "q".bold().yellow(), "Exit application");
    println!("{} - {}", "a".bold().yellow(), "Add new position");
    println!(
        "{} {} - {}",
        "e".bold().yellow(),
        "[id]".bold(),
        "Edit position"
    );
    println!(
        "{} {} - {}",
        "d".bold().yellow(),
        "[id]".bold(),
        "Delete position"
    );
    println!("{} - {}", "n".bold().yellow(), "Show next page");
    println!("{} - {}", "n".bold().yellow(), "Show previous page");
    println!();
}

pub fn render_edit_position_help_page() {
    clear_screen().expect("clear screen");
    println!("{}\n", "Available commands:".bold());
    println!("{} - {}", "h".bold().yellow(), "Show help page");
    println!("{} - {}", "q".bold().yellow(), "Return to positions");
    println!("{} - {}", "a".bold().yellow(), "Add new order");
    println!(
        "{} {} - {}",
        "d".bold().yellow(),
        "[id]".bold(),
        "Delete order"
    );
    println!(
        "{} {} - {}",
        "n".bold().yellow(),
        "[id]".bold(),
        "Show next page"
    );
    println!("{} - {}", "n".bold().yellow(), "Show previous page");
    println!();
}

pub fn render_single_position(position: &Position) {
    let mut table = Table::new();
    table.add_row(row![
        "Id",
        "Name",
        "Amount",
        "Avg value",
        "Avg price",
        "Income"
    ]);

    table.add_row(row![
        position.id,
        position.name,
        round(position.amount).unwrap(),
        round(position.avg_value).unwrap(),
        round(position.avg_price).unwrap(),
        round(position.income).unwrap(),
    ]);

    table.printstd();
}

pub fn render_position_info(position: &Position, page: i32) {
    let mut position_table = Table::new();
    position_table.add_row(row![
        "Id",
        "Name",
        "Amount",
        "Avg value",
        "Avg price",
        "Income"
    ]);

    position_table.add_row(row![
        position.id,
        position.name,
        round(position.amount).unwrap(),
        round(position.avg_value).unwrap(),
        round(position.avg_price).unwrap(),
        round(position.income).unwrap(),
    ]);

    let mut orders_table = Table::new();

    orders_table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

    position.orders.iter().for_each(|order| {
        let order_type = match order.action {
            Action::Long => "Buy",
            Action::Short => "Sell",
        };

        let income_value = if position.action == order.action {
            String::from("-")
        } else {
            round(order.income).unwrap().to_string()
        };

        orders_table.add_row(row![
            order.id,
            order_type,
            round(order.amount).unwrap(),
            round(order.value).unwrap(),
            round(order.price).unwrap(),
            income_value
        ]);
    });

    clear_screen().expect("clear screen");

    print!("Position {} ", position.id);
    match position.action {
        Action::Long => print!("{} ", "Long".bold().green()),
        Action::Short => println!("{} ", "Short".bold().red()),
    }
    println!("{}", position.name.bold());
    position_table.printstd();

    println!(); // Gap between tables

    println!("Position {} orders:", position.id.to_string().bold());
    orders_table.printstd();
    draw_page_counter(
        page,
        get_pages_count(position.orders.len(), ORDERS_PER_PAGE),
    );
}

pub fn render_single_order(position: &Position, order: &Order) {
    let order_type = match order.action {
        Action::Long => "Buy",
        Action::Short => "Sell",
    };

    let income_value = if position.action == order.action {
        String::from("-")
    } else {
        if order.income > 0f64 {
            format!("+{}", round(order.income).unwrap())
        } else {
            round(order.income).unwrap().to_string()
        }
    };

    let mut table = Table::new();
    table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

    table.add_row(row![
        order.id,
        order_type,
        round(order.amount).unwrap(),
        round(order.value).unwrap(),
        round(order.price).unwrap(),
        income_value
    ]);

    table.printstd();
}

fn calculate_total(positions: &Vec<Position>) -> (f64, f64) {
    let mut income = 0f64;
    let mut value = 0f64;

    for pos in positions {
        income += pos.income;
        value += pos.avg_value;
    }

    (value, income)
}
