use serde::{Deserialize, Serialize};

use super::{Action, Order};

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

        let mut same_action_prices_sum = 0f64;
        let mut same_action_orders_count = 0f64;

        orders.iter().for_each(|order| {
            if pos_type == order.action {
                amount += order.amount;
                value += order.value;

                same_action_prices_sum += order.price;
                same_action_orders_count += 1f64;
            } else {
                amount -= order.amount;
                value -= order.value;
            }
        });

        let avg_price = same_action_prices_sum / same_action_orders_count;
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
