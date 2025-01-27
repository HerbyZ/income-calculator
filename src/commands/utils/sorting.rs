use crate::models::Position;

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Descending,
    Ascending,
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Id(SortDirection),
    AvgValue(SortDirection),
    LastChange(SortDirection),
    AvgPrice(SortDirection),
    Income(SortDirection),
}

pub fn get_sorted_positions(positions: &Vec<Position>, sort_by: &SortBy) -> Vec<Position> {
    let mut positions = positions.clone();

    match sort_by {
        SortBy::Id(direction) => positions.sort_by(|first, second| match direction {
            SortDirection::Descending => first.id.cmp(&second.id),
            SortDirection::Ascending => second.id.cmp(&first.id),
        }),
        SortBy::AvgValue(direction) => positions.sort_by(|first, second| match direction {
            SortDirection::Descending => first.avg_value.total_cmp(&second.avg_price),
            SortDirection::Ascending => second.avg_value.total_cmp(&first.avg_value),
        }),
        SortBy::Income(direction) => positions.sort_by(|first, second| match direction {
            SortDirection::Descending => first.income.total_cmp(&second.income),
            SortDirection::Ascending => second.income.total_cmp(&first.income),
        }),
        SortBy::LastChange(direction) => positions.sort_by(|first, second| match direction {
            SortDirection::Descending => first.edited_at.cmp(&second.edited_at),
            SortDirection::Ascending => second.edited_at.cmp(&first.edited_at),
        }),
        SortBy::AvgPrice(direction) => positions.sort_by(|first, second| match direction {
            SortDirection::Descending => first.avg_price.total_cmp(&second.avg_price),
            SortDirection::Ascending => second.avg_price.total_cmp(&first.avg_price),
        }),
    }

    positions
}
