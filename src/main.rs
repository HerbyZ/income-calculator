pub mod commands;
pub mod models;
pub mod storage;
pub mod utils;

use commands::CommandHandler;
use models::Position;
use utils::console::wait_for_enter;

fn exit_with_error(error: String) {
    println!("ERROR: {}", error);
    std::process::exit(1);
}

fn main() {
    if let Err(error) = storage::initialize_storage() {
        return exit_with_error(error);
    }

    let initial_positions = match storage::load_positions() {
        Ok(positions) => positions,
        Err(error) => return exit_with_error(error),
    };

    let mut command_handler = CommandHandler::new(initial_positions);
    let stdin = std::io::stdin();

    command_handler.show_ui();

    loop {
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).expect("read command from stdin");

        let command_result: Result<(), String> = command_handler.handle_command(cmd);

        if let Some(error) = command_result.err() {
            println!("ERROR: {}", error);

            if let Err(console_error) = wait_for_enter() {
                exit_with_error(console_error);
            }
        }

        command_handler.show_ui();
    }
}
