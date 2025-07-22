use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::Path};

use crate::{constants::OPTIONS_FILE_PATH, exit_with_error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub positions_per_page: i32,
    pub orders_per_page: i32,
    pub storage_file_path: String,
}

impl Options {
    fn to_json_string(&self) -> String {
        json!(&self).to_string()
    }
}

pub fn initialize_options() -> Result<(), String> {
    if Path::new(OPTIONS_FILE_PATH).exists() {
        return Ok(());
    };

    let default_options = Options {
        positions_per_page: 10,
        orders_per_page: 10,
        storage_file_path: String::from("./storage.json"),
    };

    match fs::write(OPTIONS_FILE_PATH, default_options.to_json_string()) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to write initial storage file")),
    }
}

pub fn get_options() -> Options {
    let file_content = match std::fs::read_to_string(OPTIONS_FILE_PATH) {
        Ok(content) => content,
        Err(_) => exit_with_error(String::from("Failed to read options file")),
    };

    match serde_json::from_str::<Options>(&file_content) {
        Ok(data) => data,
        Err(_) => exit_with_error(String::from("Failed to deserialize options")),
    }
}
