pub fn select_items_for_page<T>(mut items: Vec<T>, page: i32, items_per_page: i32) -> Vec<T>
where
    T: Clone,
{
    let split_index: usize = (items_per_page * (page - 1)).try_into().unwrap();
    let splitted_items = items.split_off(split_index);

    let mut result: Vec<T> = vec![];
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
