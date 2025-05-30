pub mod commands;
pub mod constants;
pub mod models;
pub mod storage;
pub mod utils;

use commands::CommandHandler;
use models::Position;
use utils::console::{ask_confirmation, print_error, wait_for_enter, ConfirmationStatus};

fn exit_with_error(error: String) -> ! {
    print_error(error);
    std::process::exit(1);
}

fn handle_load_initial_positions_error(error: String) -> Vec<Position> {
    print_error(error);
    let confirmation_message =
"Perhaps, the version of app has changed. You can reinitialize storage (all data will be lost) or 
try to fix data in file yourself\ny - Reinitialize storage\nn - Exit without changes\n(Default: n)";
    let confirmation_status =
        match ask_confirmation(confirmation_message, ConfirmationStatus::Rejected) {
            Ok(value) => value,
            Err(error) => exit_with_error(error),
        };

    if confirmation_status == ConfirmationStatus::Confirmed {
        if let Err(error) = storage::reinitialize_storage() {
            exit_with_error(error)
        }
    } else {
        std::process::exit(0)
    }

    match storage::load_positions() {
        Ok(value) => value,
        Err(error) => exit_with_error(error),
    }
}

fn main() {
    if let Err(error) = storage::initialize_storage() {
        exit_with_error(error);
    }

    let initial_positions = match storage::load_positions() {
        Ok(positions) => positions,
        Err(error) => handle_load_initial_positions_error(error),
    };

    let mut command_handler = CommandHandler::new(&initial_positions);
    let stdin = std::io::stdin();

    command_handler.show_ui();

    loop {
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).expect("read command from stdin");

        let command_result: Result<(), String> = command_handler.handle_command(cmd);

        if let Some(error) = command_result.err() {
            print_error(error);

            if let Err(console_error) = wait_for_enter() {
                exit_with_error(console_error);
            }
        }

        command_handler.show_ui();
    }
}
