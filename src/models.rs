use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub id: i32,
    pub position_type: OrderType,
    pub name: String,
    pub amount: f64,
    pub value: f64,
    pub avg_price: f64,
    pub income: f64,
    pub orders: Vec<PositionOrder>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionOrder {
    pub id: i32,
    pub order_type: OrderType,
    pub amount: f64,
    pub value: f64,
    pub price: f64,
    pub income: f64,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum OrderType {
    Long,
    Short,
}

impl Position {
    pub fn new(id: i32, name: String, mut orders: Vec<PositionOrder>) -> Position {
        orders.sort_by(|first, second| first.id.cmp(&second.id));
        let pos_type = orders.first().unwrap().clone().order_type;

        let mut amount = 0f64;
        let mut value = 0f64;

        orders.iter().for_each(|order| {
            if pos_type == order.order_type {
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
            if pos_type != order.order_type {
                income += avg_price - order.price;
            }
        });

        Position {
            id,
            name,
            position_type: pos_type,
            amount,
            avg_price,
            income,
            value,
            orders,
        }
    }
}
