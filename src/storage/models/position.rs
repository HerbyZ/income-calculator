use serde::{Deserialize, Serialize};

use crate::models::{Order, Position};

use super::{ActionStorageModel, FromModel, OrderStorageModel, ToModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionStorageModel {
    pub id: i32,
    pub action: ActionStorageModel,
    pub name: String,
    pub orders: Vec<OrderStorageModel>,
}

impl ToModel<Position> for PositionStorageModel {
    fn to_model(&self) -> Result<Position, String> {
        let mut orders = self.orders.to_vec();
        orders.sort_by(|first, second| first.id.cmp(&second.id));

        let first_order_model = match orders.first() {
            Some(model) => model,
            None => {
                return Err(format!(
                    "Failed to parse position {} orders, perhaps it's empty",
                    self.id
                ))
            }
        };
        let first_order = Order {
            id: first_order_model.id,
            action: match first_order_model.action.to_model() {
                Ok(action) => action,
                Err(error) => return Err(error),
            },
            amount: first_order_model.amount,
            value: first_order_model.value,
            price: first_order_model.value / first_order_model.amount,
            income: 0f64,
        };

        let mut pos = Position::new(self.id, self.name.clone(), vec![first_order]);

        orders.remove(0);
        for order_models in orders {
            let action = match order_models.action.to_model() {
                Ok(action) => action,
                Err(error) => return Err(error),
            };

            pos.add_order(Order::new(
                pos.clone(),
                action,
                order_models.amount,
                order_models.value,
            ));
        }

        Ok(pos)
    }
}

impl FromModel<Position> for PositionStorageModel {
    fn from_model(model: Position) -> PositionStorageModel {
        let mut order_models = vec![];
        for order in model.orders {
            order_models.push(OrderStorageModel::from_model(order));
        }

        PositionStorageModel {
            id: model.id,
            action: ActionStorageModel::from_model(model.action),
            name: model.name,
            orders: order_models,
        }
    }
}
