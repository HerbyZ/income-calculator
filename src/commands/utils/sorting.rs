use crate::models::Position;

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Descending,
    Ascending,
}

impl ToString for SortDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Ascending => String::from("Asc"),
            Self::Descending => String::from("Desc"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Id(SortDirection),
    AvgValue(SortDirection),
    LastChange(SortDirection),
    AvgPrice(SortDirection),
    Income(SortDirection),
}

impl ToString for SortBy {
    fn to_string(&self) -> String {
        match self {
            Self::Id(direction) => format!("Id ({})", direction.to_string()),
            Self::AvgValue(direction) => format!("Avg value ({})", direction.to_string()),
            Self::LastChange(direction) => format!("Last change ({})", direction.to_string()),
            Self::AvgPrice(direction) => format!("Avg price ({})", direction.to_string()),
            Self::Income(direction) => format!("Income ({})", direction.to_string()),
        }
    }
}

pub struct PositionsSorter {
    pub sort_by: SortBy,
    pub move_closed_to_bottom: bool,
}

impl PositionsSorter {
    pub fn sort(&self, positions: &Vec<Position>) -> Vec<Position> {
        let mut positions = positions.clone();

        match self.sort_by {
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

        if self.move_closed_to_bottom {
            let mut positions_sorted_by_status = vec![];
            let mut closed_positions = vec![];

            positions
                .iter()
                .for_each(|pos| match pos.avg_value == 0f64 {
                    true => positions_sorted_by_status.push(pos.clone()),
                    false => closed_positions.push(pos.clone()),
                });

            positions_sorted_by_status.append(&mut closed_positions);
            return positions_sorted_by_status;
        }

        positions
    }
}
