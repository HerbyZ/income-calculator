use colored::Colorize;
use prettytable::{row, Table};

use crate::models::Position;
use crate::utils::console::clear_screen;
use crate::utils::math::round;
use crate::utils::pagination::select_items_for_page;

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
        table.add_row(row!["Id", "Name", "Amount", "Value", "Avg price", "Income"]);

        let mut reversed_positions = self.positions.clone();
        reversed_positions.reverse();

        let positions_to_draw =
            select_items_for_page(reversed_positions, self.page, ITEMS_PER_PAGE);

        positions_to_draw.iter().for_each(|position| {
            table.add_row(row![
                position.id,
                position.name,
                round(position.amount).unwrap(),
                round(position.value).unwrap(),
                round(position.avg_price).unwrap(),
                round(position.income).unwrap(),
            ]);
        });

        table.printstd();
        self.draw_page_counter();
    }

    pub fn draw_single_position(&self, id: i32) -> Result<(), String> {
        let position_option = self.positions.iter().find(|pos| pos.id == id);
        if position_option.is_none() {
            return Err(format!("Position with id {} not found", id));
        }

        let position = position_option.unwrap();

        let mut table = Table::new();
        table.add_row(row!["Id", "Name", "Amount", "Value", "Avg price", "Income"]);

        table.add_row(row![
            position.id,
            position.name,
            round(position.amount).unwrap(),
            round(position.value).unwrap(),
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
        let max_page = self.get_pages_count();
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
        println!("{} - {}", "h".bold().black().on_white(), "Show help page");
        println!("{} - {}", "q".bold().black().on_white(), "Exit application");
        println!("{} - {}", "a".bold().black().on_white(), "Add new position");
        println!("{} - {}", "e".bold().black().on_white(), "Edit position");
        println!("{} - {}", "d".bold().black().on_white(), "Delete position");
        println!("{} - {}", "n".bold().black().on_white(), "Show next page");
        println!(
            "{} - {}",
            "n".bold().black().on_white(),
            "Show previous page"
        );

        println!("\n{}", "Press Enter to continue...".italic().bright_black());
    }

    fn draw_page_counter(&self) {
        let pages_count: f64 = self.get_pages_count();
        print!("Page ");

        println!(
            "{}{}{}",
            self.page.to_string().bold().black().on_white(),
            "/".black().on_white(),
            pages_count.to_string().bold().black().on_white()
        );
    }

    fn get_pages_count(&self) -> f64 {
        (self.positions.len() as f64 / ITEMS_PER_PAGE as f64).ceil()
    }
}
