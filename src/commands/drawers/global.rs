use colored::Colorize;
use prettytable::{row, Table};

use crate::models::Position;
use crate::utils::console::clear_screen;
use crate::utils::math::round;
use crate::utils::pagination::{draw_page_counter, get_pages_count, select_items_for_page};

const ITEMS_PER_PAGE: i32 = 10;

pub struct GlobalDrawer {
    page: i32,
    pub positions: Vec<Position>,
}

impl GlobalDrawer {
    pub fn new(positions: Vec<Position>) -> GlobalDrawer {
        GlobalDrawer { positions, page: 1 }
    }

    pub fn render_positions_table(&self) {
        clear_screen().expect("clear screen");

        let mut table = Table::new();
        table.add_row(row![
            "Id",
            "Name",
            "Amount",
            "Avg value",
            "Avg price",
            "Income"
        ]);

        let mut reversed_positions = self.positions.clone();
        reversed_positions.reverse();

        let positions_to_draw =
            select_items_for_page(reversed_positions, self.page, ITEMS_PER_PAGE);

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

        table.printstd();
        draw_page_counter(
            self.page,
            get_pages_count(self.positions.len(), ITEMS_PER_PAGE),
        );
    }

    pub fn draw_single_position(&self, id: i32) -> Result<(), String> {
        let position_option = self.positions.iter().find(|pos| pos.id == id);
        if position_option.is_none() {
            return Err(format!("Position with id {} not found", id));
        }

        let position = position_option.unwrap();

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

        Ok(())
    }

    pub fn previous_page(&mut self) -> Result<(), String> {
        if self.page == 1 {
            Err(String::from("Already at first page"))
        } else {
            self.page -= 1;
            Ok(())
        }
    }

    pub fn next_page(&mut self) -> Result<(), String> {
        let max_page = get_pages_count(self.positions.len(), ITEMS_PER_PAGE);
        if (self.page + 1) as f64 > max_page {
            Err(String::from("Already at last page"))
        } else {
            self.page += 1;
            Ok(())
        }
    }

    pub fn draw_help_tooltip(&self) {
        println!("{}", "Type 'h' for help".italic().bright_black());
    }

    pub fn draw_help_page(&self) {
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
}
