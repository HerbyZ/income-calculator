use super::{Action, Position};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub action: Action,
    pub amount: f64,
    pub value: f64,
    pub price: f64,
    pub income: f64,
}

impl Order {
    pub fn new(pos: Position, action: Action, amount: f64, value: f64) -> Order {
        let price = value / amount;
        println!("{price}");
        println!("{}", pos.avg_price);
        let income = (price - pos.avg_price) * amount;

        let mut pos_orders_clone = pos.orders.clone();
        pos_orders_clone.sort_by(|first, second| first.id.cmp(&second.id));
        let id = pos_orders_clone.last().unwrap().id + 1;

        Order {
            id,
            action,
            amount,
            value,
            price,
            income,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, Order, Position};

    #[test]
    fn test_order_new() {
        let manual_order = Order {
            id: 0,
            action: Action::Long,
            amount: 10f64,
            value: 100f64,
            price: 10f64,
            income: 0f64,
        };
        let pos = Position::new(0, String::from("MOCK"), vec![manual_order]);

        // Create order with Order::new()
        let order = Order::new(pos, Action::Short, 10f64, 200f64);
        assert_eq!(order.id, 1);
        assert_eq!(order.price, 20f64);
        assert_eq!(order.income, 100f64);
    }
}
