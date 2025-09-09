pub mod model;

use serde_json::json;
use std::{fs, path::Path};

use model::Options;

use crate::{constants::OPTIONS_FILE_PATH, exit_with_error};

pub fn initialize_options() -> Result<(), String> {
    if Path::new(OPTIONS_FILE_PATH).exists() {
        return Ok(());
    };

    let default_options_contents = json!(Options {
        positions_per_page: 10,
        orders_per_page: 10,
        hide_closed_positions: false,
        storage_file_path: String::from("./storage.json"),
    })
    .to_string();

    match fs::write(OPTIONS_FILE_PATH, default_options_contents) {
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
        Ok(options_value) => options_value,
        Err(_) => exit_with_error(String::from("Failed to deserialize options")),
    }
}
