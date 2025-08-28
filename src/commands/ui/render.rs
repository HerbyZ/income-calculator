use colored::Colorize;
use prettytable::{cell, color, row, Attr, Cell, Row, Table};
use std::cmp::Ordering;

use crate::models::{Action, Order, Position};
use crate::options::get_options;
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
        "Income",
        "%",
        "Status"
    ]);

    let mut reversed_positions = positions.to_vec();
    reversed_positions.reverse();

    let positions_per_page = get_options().positions_per_page;
    let positions_to_draw = select_items_for_page(reversed_positions, page, positions_per_page);

    positions_to_draw.iter().for_each(|position| {
        table.add_row(Row::new(vec![
            cell!(position.id),
            cell!(position.name),
            cell!(round(position.amount).unwrap()),
            cell!(round(position.avg_value).unwrap()),
            cell!(round(position.avg_price).unwrap()),
            get_styled_income_cell(round(position.income).unwrap(), None),
            get_styled_income_cell(
                round(position.calculate_income_percent()).unwrap(),
                Some(String::from("%")),
            ),
            get_status_cell(position),
        ]));
    });

    // Add total row
    let (value, income) = calculate_total(positions);
    table.add_row(Row::new(vec![
        cell!("Total"),
        cell!("-"),
        cell!("-"),
        cell!(round(value).unwrap()),
        cell!("-"),
        get_styled_income_cell(round(income).unwrap(), None),
        cell!("-"),
    ]));

    table.printstd();

    let positions_per_page = get_options().positions_per_page;
    draw_page_counter(page, get_pages_count(positions.len(), positions_per_page));
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
    println!("{} - {}", "cs".bold().yellow(), "Change sorting");
    println!("{} - {}", "n".bold().yellow(), "Show next page");
    println!("{} - {}", "p".bold().yellow(), "Show previous page");
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

    table.add_row(Row::new(vec![
        cell!(position.id),
        cell!(position.name),
        cell!(round(position.amount).unwrap()),
        cell!(round(position.avg_value).unwrap()),
        cell!(round(position.avg_price).unwrap()),
        get_styled_income_cell(round(position.income).unwrap(), None),
    ]));

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

    position_table.add_row(Row::new(vec![
        cell!(position.id),
        cell!(position.name),
        cell!(round(position.amount).unwrap()),
        cell!(round(position.avg_value).unwrap()),
        cell!(round(position.avg_price).unwrap()),
        get_styled_income_cell(round(position.income).unwrap(), None),
    ]));

    let mut orders_table = Table::new();

    orders_table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

    position.orders.iter().for_each(|order| {
        let order_type = match order.action {
            Action::Long => "Buy",
            Action::Short => "Sell",
        };

        let income_cell = if position.action == order.action {
            cell!(String::from("-"))
        } else {
            get_styled_income_cell(round(order.income).unwrap(), None)
        };

        orders_table.add_row(Row::new(vec![
            cell!(order.id),
            cell!(order_type),
            cell!(round(order.amount).unwrap()),
            cell!(round(order.value).unwrap()),
            cell!(round(order.price).unwrap()),
            income_cell,
        ]));
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

    let orders_per_page = get_options().orders_per_page;
    draw_page_counter(
        page,
        get_pages_count(position.orders.len(), orders_per_page),
    );
}

pub fn render_single_order(position: &Position, order: &Order) {
    let order_type = match order.action {
        Action::Long => "Buy",
        Action::Short => "Sell",
    };

    let income_cell = if position.action == order.action {
        cell!(String::from("-"))
    } else {
        get_styled_income_cell(round(order.income).unwrap(), None)
    };

    let mut table = Table::new();
    table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

    table.add_row(Row::new(vec![
        cell!(order.id),
        cell!(order_type),
        cell!(round(order.amount).unwrap()),
        cell!(round(order.value).unwrap()),
        cell!(round(order.price).unwrap()),
        income_cell,
    ]));

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

fn get_styled_income_cell(income: f64, postfix: Option<String>) -> Cell {
    let cell_value = match postfix {
        Some(postfix) => format!("{}{}", income, postfix),
        None => income.to_string(),
    };

    match income.total_cmp(&0f64) {
        Ordering::Equal => cell!(cell_value),
        Ordering::Greater => {
            cell!(format!("+{}", cell_value)).with_style(Attr::ForegroundColor(color::GREEN))
        }
        Ordering::Less => cell!(cell_value).with_style(Attr::ForegroundColor(color::RED)),
    }
}

fn get_status_cell(position: &Position) -> Cell {
    if position.amount == 0f64 {
        cell!("Closed").with_style(Attr::ForegroundColor(color::BRIGHT_BLACK))
    } else {
        cell!("Active")
    }
}
