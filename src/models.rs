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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionOrder {
    pub id: i32,
    pub order_type: OrderType,
    pub amount: f64,
    pub value: f64,
    pub price: f64,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum OrderType {
    Long,
    Short,
}
