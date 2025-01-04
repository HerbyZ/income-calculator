use colored::Colorize;
use prettytable::{row, Table};

use crate::models::{Action, Position};
use crate::utils::console::clear_screen;
use crate::utils::math::round;

const ITEMS_PER_PAGE: i32 = 10;

#[derive(Clone)]
pub struct PositionDrawer {
    position: Position,
    page: i32,
}

impl PositionDrawer {
    pub fn new(position: Position) -> PositionDrawer {
        PositionDrawer { position, page: 1 }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn render_position_info(self) {
        let mut table = Table::new();

        table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

        self.position.orders.iter().for_each(|order| {
            let order_type = match order.action {
                Action::Long => "Buy",
                Action::Short => "Sell",
            };

            let income_value = if self.position.action == order.action {
                String::from("-")
            } else {
                round(order.income).unwrap().to_string()
            };

            table.add_row(row![
                order.id,
                order_type,
                round(order.amount).unwrap(),
                round(order.value).unwrap(),
                round(order.price).unwrap(),
                income_value
            ]);
        });

        clear_screen().expect("clear screen");

        print!("Position {} ", self.position.id);
        match self.position.action {
            Action::Long => print!("{} ", "Long".bold().green()),
            Action::Short => todo!("{} ", "Short".bold().red()),
        }
        println!("{}", self.position.name.bold());
        table.printstd();
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
        let max_page = self.get_orders_count();
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

    fn get_orders_count(&self) -> f64 {
        (self.position.orders.len() as f64 / ITEMS_PER_PAGE as f64).ceil()
    }
}
