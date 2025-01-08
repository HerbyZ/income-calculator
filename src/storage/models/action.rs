use serde::{Deserialize, Serialize};

use crate::models::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStorageModel {
    L,
    S,
}

impl ActionStorageModel {
    pub fn to_model(&self) -> Action {
        match self {
            ActionStorageModel::L => Action::Long,
            ActionStorageModel::S => Action::Short,
        }
    }

    pub fn from_model(model: Action) -> ActionStorageModel {
        match model {
            Action::Long => ActionStorageModel::L,
            Action::Short => ActionStorageModel::S,
        }
    }
}
