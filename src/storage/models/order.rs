use serde::{Deserialize, Serialize};

use crate::models::Order;

use super::ActionStorageModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStorageModel {
    pub id: i32,
    pub action: ActionStorageModel,
    pub amount: f64,
    pub value: f64,
}

impl OrderStorageModel {
    pub fn from_model(model: Order) -> OrderStorageModel {
        OrderStorageModel {
            id: model.id,
            action: ActionStorageModel::from_model(model.action),
            amount: model.amount,
            value: model.value,
        }
    }
}
