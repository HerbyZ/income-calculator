use std::str::FromStr;

use colored::Colorize;

#[derive(PartialEq, Eq)]
pub enum ConfirmationStatus {
    Confirmed,
    Rejected,
}

pub fn ask_for_input<T>(question: &str) -> Result<T, String>
where
    T: FromStr,
{
    println!("{}", question);

    let mut input = String::new();
    let input_result = std::io::stdin().read_line(&mut input);
    if input_result.is_err() {
        return Err(String::from("Failed to read input from console"));
    }

    match input.trim().parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Failed to parse answer '{}'", input.trim())),
    }
}

pub fn ask_confirmation(
    question: &str,
    default: ConfirmationStatus,
) -> Result<ConfirmationStatus, String> {
    println!("{}", question);

    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return Err(String::from("Failed to read input from console"));
    }

    match input.trim().to_lowercase().as_str() {
        "y" => Ok(ConfirmationStatus::Confirmed),
        "n" => Ok(ConfirmationStatus::Rejected),
        "" => Ok(default),
        _ => Err(format!("Unsupported confirmation answer {}", input.trim())),
    }
}

pub fn wait_for_enter() -> Result<(), String> {
    println!("{}", "Press enter to continue...".italic().bright_black());
    let mut s = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut s) {
        return Err(String::from("Failed to read input from console"));
    }

    Ok(())
}

pub fn clear_screen() -> std::io::Result<()> {
    let term = console::Term::stdout();
    term.clear_screen()
}

pub fn print_error(error: String) {
    println!(
        "{}{}",
        "ERROR: ".bold().white().on_red(),
        error.bold().white().on_red()
    );
}
