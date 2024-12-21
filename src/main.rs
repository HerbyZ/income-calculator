pub mod commandhandler;
pub mod drawer;
pub mod storage;
pub mod utils;

use commandhandler::CommandHandler;
use drawer::Drawer;
use storage::Position;

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

    command_handler.drawer.draw_table();

    loop {
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).expect("read command from stdin");

        let command_result: Result<(), String> = match cmd.trim() {
            "q" => break,
            "n" => command_handler.handle_next_page(),
            "p" => command_handler.handle_previous_page(),
            "a" => command_handler.handle_add_position(),
            "c" => command_handler.handle_close_position(),
            "d" => command_handler.handle_delete_position(),
            _ => {
                command_handler.drawer.draw_table();
                continue;
            }
        };

        if let Some(err) = command_result.err() {
            println!("{}", err);
        }

        command_handler.drawer.draw_table();
    }
}
