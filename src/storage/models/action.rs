use serde::{Deserialize, Serialize};

use crate::models::Action;

use super::{FromModel, ToModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStorageModel {
    L,
    S,
}

impl ToModel<Action> for ActionStorageModel {
    fn to_model(&self) -> Result<Action, String> {
        match self {
            ActionStorageModel::L => Ok(Action::Long),
            ActionStorageModel::S => Ok(Action::Short),
        }
    }
}
impl FromModel<Action> for ActionStorageModel {
    fn from_model(model: Action) -> ActionStorageModel {
        match model {
            Action::Long => ActionStorageModel::L,
            Action::Short => ActionStorageModel::S,
        }
    }
}
