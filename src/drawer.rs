use colored::Colorize;
use prettytable::{row, Table};

use crate::utils::math::round;
use crate::Position;

const ITEMS_PER_PAGE: i32 = 10;

pub struct Drawer {
    page: i32,
    pub positions: Vec<Position>,
}

impl Drawer {
    pub fn new(positions: Vec<Position>) -> Drawer {
        Drawer { positions, page: 1 }
    }

    pub fn draw_table(&self) {
        clear_screen();

        let mut table = Table::new();
        table.add_row(row![
            "Id",
            "Name",
            "Amount",
            "Value",
            "Buy price",
            "Sell price",
            "Income"
        ]);

        let positions_to_draw =
            select_items_for_page(self.positions.clone(), self.page, ITEMS_PER_PAGE);

        positions_to_draw.iter().for_each(|position| {
            table.add_row(row![
                position.id,
                position.name,
                round(position.amount).unwrap(),
                round(position.value).unwrap(),
                round(position.buy_price).unwrap(),
                round(position.sell_price).unwrap(),
                round(position.income).unwrap(),
            ]);
        });

        table.printstd();
        self.draw_page_counter();
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

fn clear_screen() {
    let term = console::Term::stdout();
    term.clear_screen().expect("clear terminal screen");
}

fn select_items_for_page(
    mut items: Vec<Position>,
    page: i32,
    items_per_page: i32,
) -> Vec<Position> {
    let split_index: usize = (items_per_page * (page - 1)).try_into().unwrap();
    let splitted_items = items.split_off(split_index);

    let mut result: Vec<Position> = vec![];
    for i in 0..items_per_page - 1 {
        let index: usize = i.try_into().unwrap();

        if let Some(item) = splitted_items.get(index) {
            result.push(item.clone());
        } else {
            break;
        }
    }

    result
}
