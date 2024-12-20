pub mod global;

use crate::Position;
use global::GlobalHandler;

pub enum CommandResult {
    Ok,
    CommandNotFound,
    Error(String),
}

pub struct CommandHandler {
    global_handler: GlobalHandler,
}

impl CommandHandler {
    pub fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            global_handler: GlobalHandler::new(initial_positions),
        }
    }

    pub fn show_ui(&self) {
        self.global_handler.drawer.draw_table();
        self.global_handler.drawer.draw_help_tooltip();
    }

    pub fn handle_command(&mut self, command: String) -> Result<(), String> {
        let result = self.global_handler.handle_command(command);

        match result {
            CommandResult::Ok => Ok(()),
            CommandResult::CommandNotFound => Ok(()),
            CommandResult::Error(error) => Err(error),
        }
    }
}
