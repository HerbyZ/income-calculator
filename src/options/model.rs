use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub positions_per_page: i32,
    pub orders_per_page: i32,
    pub storage_file_path: String,
}
