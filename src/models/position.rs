use chrono::{DateTime, Utc};

use super::{Action, Order};

#[derive(Debug, Clone)]
pub struct Position {
    pub id: i32,
    pub edited_at: DateTime<Utc>,
    pub action: Action,
    pub name: String,
    pub amount: f64,
    pub avg_value: f64,
    pub avg_price: f64,
    pub income: f64,
    pub orders: Vec<Order>,
}

impl Position {
    pub fn new(id: i32, name: String, mut orders: Vec<Order>) -> Position {
        orders.sort_by(|first, second| first.id.cmp(&second.id));
        let first_order = orders.first().unwrap();

        let mut pos = Position {
            id,
            edited_at: Utc::now(),
            name,
            action: first_order.action,
            amount: 0f64,
            avg_price: 0f64,
            income: 0f64,
            avg_value: 0f64,
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

        let mut new_orders = self.orders.to_vec();
        new_orders.push(order);

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
                new_income += order.income;
            }
        });

        self.amount = new_amount;
        self.avg_value = new_amount * self.avg_price;
        self.income = new_income;
        self.orders = new_orders;
    }

    pub fn remove_order(&mut self, order_id: i32) -> Result<(), String> {
        if order_id == 0 {
            return Err(String::from("Cannot remove first order"));
        }

        let order_index = match self.orders.iter().position(|order| order.id == order_id) {
            Some(order) => order,
            None => {
                return Err(format!(
                    "Order with id {} not found in position {}",
                    order_id, self.id
                ))
            }
        };

        let mut new_orders = self.orders.to_vec();
        new_orders.remove(order_index);

        self.orders = vec![];
        new_orders
            .iter()
            .for_each(|order| self.add_order(order.to_owned()));

        Ok(())
    }

    pub fn calculate_income_percent(&self) -> f64 {
        // TODO: Calculate position income percent
        0f64
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

        position.add_order(Order::new(&position, Action::Long, 10f64, 50f64));
        position.add_order(Order::new(&position, Action::Short, 10f64, 50f64));

        /*
        After adding these orders, 'amount' should be equal to initial;
        'avg_price', 'avg_value' and 'income' should be recalculated to 7.5, 75 and -25.
        */

        assert_eq!(position.amount, 10f64);
        assert_eq!(position.avg_value, 75f64);
        assert_eq!(position.avg_price, 7.5f64);
        assert_eq!(position.income, -25f64);
    }

    #[test]
    fn test_position_remove_order() {
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

        let tested_order_id = 2;
        position.add_order(Order {
            id: tested_order_id,
            action: Action::Short,
            amount: 10f64,
            value: 50f64,
            price: 5f64,
            income: 5f64,
        });

        // Remove last order and check recalculation
        position
            .remove_order(tested_order_id)
            .expect("Remove test order");

        assert_eq!(position.amount, 20f64);
        assert_eq!(position.avg_value, 150f64);
        assert_eq!(position.avg_price, 7.5f64);
        assert_eq!(position.income, 0f64);
    }

    #[test]
    fn test_position_calculate_income_percent() {
        // Test positive income
        let position = Position::new(
            1,
            String::from("MOCK"),
            vec![
                Order {
                    id: 0,
                    action: Action::Long,
                    amount: 1f64,
                    value: 100f64,
                    price: 100f64,
                    income: 0f64,
                },
                Order {
                    id: 1,
                    action: Action::Short,
                    amount: 1f64,
                    value: 200f64,
                    price: 200f64,
                    income: 100f64,
                },
            ],
        );

        assert_eq!(position.calculate_income_percent(), 100f64);

        // Test negative income
        let position = Position::new(
            1,
            String::from("MOCK"),
            vec![
                Order {
                    id: 0,
                    action: Action::Long,
                    amount: 1f64,
                    value: 200f64,
                    price: 200f64,
                    income: 0f64,
                },
                Order {
                    id: 1,
                    action: Action::Short,
                    amount: 1f64,
                    value: 100f64,
                    price: 100f64,
                    income: 100f64,
                },
            ],
        );

        assert_eq!(position.calculate_income_percent(), -50f64);
    }
}
