use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub id: i32,
    pub action: Action,
    pub name: String,
    pub amount: f64,
    pub value: f64,
    pub avg_price: f64,
    pub income: f64,
    pub orders: Vec<Order>,
}

impl Position {
    pub fn new(id: i32, name: String, mut orders: Vec<Order>) -> Position {
        orders.sort_by(|first, second| first.id.cmp(&second.id));
        let pos_type = orders.first().unwrap().clone().action;

        let mut amount = 0f64;
        let mut value = 0f64;

        orders.iter().for_each(|order| {
            if pos_type == order.action {
                amount += order.amount;
                value += order.value;
            } else {
                amount -= order.amount;
                value -= order.value;
            }
        });

        let avg_price = value / amount;
        let mut income = 0f64;

        orders.iter().for_each(|order| {
            if pos_type != order.action {
                income += avg_price - order.price;
            }
        });

        Position {
            id,
            name,
            action: pos_type,
            amount,
            avg_price,
            income,
            value,
            orders,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub action: Action,
    pub amount: f64,
    pub value: f64,
    pub price: f64,
    pub income: f64,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Long,
    Short,
}

impl Action {
    pub fn from_string(string: String) -> Result<Action, String> {
        match string.to_lowercase().as_str() {
            "l" | "long" => Ok(Action::Long),
            "s" | "short" => Ok(Action::Short),
            _ => {
                return Err(format!(
                    "'{}' is not valid position type (long/short)",
                    string
                ))
            }
        }
    }
}
