use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub positions_per_page: i32,
    pub hide_closed_positions: bool,
    pub orders_per_page: i32,
    pub storage_file_path: String,
}
