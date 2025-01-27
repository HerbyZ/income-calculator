pub mod action;
pub mod order;
pub mod position;
pub mod sort_by;
pub mod storage;

pub use action::ActionStorageModel;
pub use order::OrderStorageModel;
pub use position::PositionStorageModel;

pub trait ToModel<T> {
    fn to_model(&self) -> Result<T, String>;
}

pub trait FromModel<T> {
    fn from_model(model: T) -> Self;
}
