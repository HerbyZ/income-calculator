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
        let first_order = orders.first().unwrap().clone();

        let mut pos = Position {
            id,
            name,
            action: first_order.action,
            amount: 0f64,
            avg_price: 0f64,
            income: 0f64,
            value: 0f64,
            orders: vec![],
        };

        orders
            .iter()
            .for_each(|order| pos.add_order(order.to_owned()));

        pos
    }

    pub fn add_order(&mut self, order: Order) {
        let mut same_action_prices_sum = 0f64;
        let mut same_action_orders_count = 0f64;

        let mut new_orders = self.orders.clone();
        new_orders.push(order.clone());

        let mut new_amount = 0f64;

        new_orders.iter().for_each(|order| {
            if self.action == order.action {
                new_amount += order.amount;

                same_action_prices_sum += order.price;
                same_action_orders_count += 1f64;
            } else {
                new_amount -= order.amount;
            }
        });

        self.avg_price = same_action_prices_sum / same_action_orders_count;

        let mut new_income = 0f64;

        new_orders.iter().for_each(|order| {
            if self.action != order.action {
                new_income += (order.price - self.avg_price) * order.amount;
            }
        });

        self.amount = new_amount;
        self.value = new_amount * self.avg_price;
        self.income = new_income;
        self.orders = new_orders;
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, Order, Position};

    #[test]
    fn test_position_add_order() {
        let first_order = Order {
            id: 0,
            action: Action::Long,
            amount: 10f64,
            value: 100f64,
            price: 10f64,
            income: 0f64,
        };

        let mut position = Position::new(0, String::from("MOCK"), vec![first_order]);

        position.add_order(Order {
            id: 1,
            action: Action::Long,
            amount: 10f64,
            value: 50f64,
            price: 5f64,
            income: 0f64,
        });
        position.add_order(Order {
            id: 2,
            action: Action::Short,
            amount: 10f64,
            value: 50f64,
            price: 5f64,
            income: 5f64,
        });

        /*
        After adding these orders, 'amount' and 'value' should be equal to initial;
        'avg_price' and 'income' should be recalculated to 7.5 and -25.
        */

        assert_eq!(position.amount, 10f64);
        assert_eq!(position.value, 100f64);
        assert_eq!(position.avg_price, 7.5f64);
        assert_eq!(position.income, -25f64);
    }
}
