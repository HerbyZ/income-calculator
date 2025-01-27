use serde::{Deserialize, Serialize};

use super::{FromModel, ToModel};
use crate::commands::utils::sorting::{SortBy, SortDirection};

#[derive(Debug, Serialize, Deserialize)]
pub enum SortDirectionStorageModel {
    A,
    D,
}

impl ToModel<SortDirection> for SortDirectionStorageModel {
    fn to_model(&self) -> Result<SortDirection, String> {
        match self {
            SortDirectionStorageModel::A => Ok(SortDirection::Ascending),
            SortDirectionStorageModel::D => Ok(SortDirection::Descending),
        }
    }
}

impl FromModel<SortDirection> for SortDirectionStorageModel {
    fn from_model(model: SortDirection) -> Self {
        match model {
            SortDirection::Ascending => Self::A,
            SortDirection::Descending => Self::D,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SortByStorageModel {
    Id(SortDirectionStorageModel),
    AvgValue(SortDirectionStorageModel),
    LastChange(SortDirectionStorageModel),
    AvgPrice(SortDirectionStorageModel),
    Income(SortDirectionStorageModel),
}

impl Default for SortByStorageModel {
    fn default() -> Self {
        SortByStorageModel::LastChange(SortDirectionStorageModel::D)
    }
}

impl ToModel<SortBy> for SortByStorageModel {
    fn to_model(&self) -> Result<SortBy, String> {
        match self {
            Self::Id(direction) => Ok(SortBy::Id(direction.to_model().unwrap())),
            Self::AvgPrice(direction) => Ok(SortBy::AvgPrice(direction.to_model().unwrap())),
            Self::AvgValue(direction) => Ok(SortBy::AvgValue(direction.to_model().unwrap())),
            Self::Income(direction) => Ok(SortBy::Income(direction.to_model().unwrap())),
            Self::LastChange(direction) => Ok(SortBy::LastChange(direction.to_model().unwrap())),
        }
    }
}

impl FromModel<SortBy> for SortByStorageModel {
    fn from_model(model: SortBy) -> Self {
        match model {
            SortBy::AvgPrice(direction) => {
                Self::AvgPrice(SortDirectionStorageModel::from_model(direction))
            }
            SortBy::AvgValue(direction) => {
                Self::AvgValue(SortDirectionStorageModel::from_model(direction))
            }
            SortBy::Income(direction) => {
                Self::Income(SortDirectionStorageModel::from_model(direction))
            }
            SortBy::LastChange(direction) => {
                Self::LastChange(SortDirectionStorageModel::from_model(direction))
            }
            SortBy::Id(direction) => Self::Id(SortDirectionStorageModel::from_model(direction)),
        }
    }
}
