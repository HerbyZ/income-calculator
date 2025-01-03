use super::Action;
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
