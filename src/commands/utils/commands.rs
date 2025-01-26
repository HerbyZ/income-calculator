use std::str::FromStr;

use crate::utils::console::ask_for_input;

pub fn parse_arg_or_get_from_input<T>(arg: Option<&String>, question: &str) -> Result<T, String>
where
    T: FromStr,
{
    if arg.is_some() {
        let arg = arg.unwrap();
        match arg.trim().parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => return Err(format!("Failed to parse answer '{}'", arg.trim())),
        }
    } else {
        match ask_for_input::<T>(question) {
            Ok(value) => Ok(value),
            Err(error) => return Err(error),
        }
    }
}
