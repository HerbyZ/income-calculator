use colored::Colorize;
use prettytable::{row, Table};

use crate::models::{Action, Order, Position};
use crate::utils::console::clear_screen;
use crate::utils::math::round;
use crate::utils::pagination::{draw_page_counter, get_pages_count};

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
            self.position.id,
            self.position.name,
            round(self.position.amount).unwrap(),
            round(self.position.avg_value).unwrap(),
            round(self.position.avg_price).unwrap(),
            round(self.position.income).unwrap(),
        ]);

        let mut orders_table = Table::new();

        orders_table.add_row(row!["Id", "Type", "Amount", "Value", "Price", "Income"]);

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

        print!("Position {} ", self.position.id);
        match self.position.action {
            Action::Long => print!("{} ", "Long".bold().green()),
            Action::Short => todo!("{} ", "Short".bold().red()),
        }
        println!("{}", self.position.name.bold());
        position_table.printstd();

        println!("Position {} orders:", self.position.id.to_string().bold());
        orders_table.printstd();
        draw_page_counter(
            self.page,
            get_pages_count(self.position.orders.len(), ITEMS_PER_PAGE),
        );
    }

    pub fn render_single_order_info(&self, order: Order) {
        let order_type = match order.action {
            Action::Long => "Buy",
            Action::Short => "Sell",
        };

        let income_value = if self.position.action == order.action {
            String::from("-")
        } else {
            round(order.income).unwrap().to_string()
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

    pub fn previous_page(&mut self) -> Result<(), String> {
        if self.page == 1 {
            Err(String::from("Already at first page"))
        } else {
            self.page -= 1;
            Ok(())
        }
    }

    pub fn next_page(&mut self) -> Result<(), String> {
        let max_page = get_pages_count(self.position.orders.len(), ITEMS_PER_PAGE);
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
}
