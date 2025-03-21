use serde::{Deserialize, Serialize};

use super::super::Storage;
use super::sort_by::SortByStorageModel;
use super::{FromModel, PositionStorageModel, ToModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageModel {
    #[serde(default)]
    pub sort_positions_by: SortByStorageModel,

    positions: Vec<PositionStorageModel>,
}

impl FromModel<Storage> for StorageModel {
    fn from_model(model: Storage) -> Self {
        let mut position_models = vec![];
        for pos in model.positions {
            position_models.push(PositionStorageModel::from_model(pos));
        }

        Self {
            positions: position_models,
            sort_positions_by: SortByStorageModel::from_model(model.sort_positions_by),
        }
    }
}

impl ToModel<Storage> for StorageModel {
    fn to_model(&self) -> Result<Storage, String> {
        let mut positions = vec![];
        for pos_model in &self.positions {
            positions.push(match pos_model.to_model() {
                Ok(value) => value,
                Err(error) => return Err(error),
            });
        }

        Ok(Storage {
            positions,
            sort_positions_by: match self.sort_positions_by.to_model() {
                Ok(value) => value,
                Err(error) => return Err(error),
            },
        })
    }
}
