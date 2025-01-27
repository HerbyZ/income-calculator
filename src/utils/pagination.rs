use colored::Colorize;

pub fn select_items_for_page<T>(mut items: Vec<T>, page: i32, items_per_page: i32) -> Vec<T>
where
    T: Clone,
{
    let split_index: usize = (items_per_page * (page - 1)).try_into().unwrap();
    let splitted_items = items.split_off(split_index);

    let mut result: Vec<T> = vec![];
    for i in 0..items_per_page {
        let index: usize = i.try_into().unwrap();

        if let Some(item) = splitted_items.get(index) {
            result.push(item.clone());
        } else {
            break;
        }
    }

    result
}

pub fn draw_page_counter(current_page: i32, pages_count: f64) {
    print!("Page ");

    println!(
        "{}{}{}",
        current_page.to_string().bold().black().on_white(),
        "/".black().on_white(),
        pages_count.to_string().bold().black().on_white()
    );
}
pub fn get_pages_count(items_length: usize, items_per_page: i32) -> f64 {
    (items_length as f64 / items_per_page as f64).ceil()
}
