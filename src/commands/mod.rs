pub mod global;
pub mod position;

use crate::Position;
use global::GlobalHandler;
use position::PositionHandler;

pub enum EditMode {
    Global,
    Position(Position),
}

pub enum CommandResult {
    Ok,
    CommandNotFound,
    ChangeEditMode(EditMode),
    Error(String),
}

pub struct CommandHandler {
    global_handler: GlobalHandler,
    position_handler: PositionHandler,
    edit_mode: EditMode,
}

const DEFAULT_EDIT_MODE: EditMode = EditMode::Global;

impl CommandHandler {
    pub fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            global_handler: GlobalHandler::new(initial_positions),
            position_handler: PositionHandler::new(),
            edit_mode: DEFAULT_EDIT_MODE,
        }
    }

    pub fn show_ui(&self) {
        self.global_handler.drawer.draw_table();
        self.global_handler.drawer.draw_help_tooltip();

        match self.edit_mode {
            EditMode::Global => println!("Global edit mode"),
            EditMode::Position(_) => println!("Position edit mode"),
        }
    }

    pub fn handle_command(&mut self, command: String) -> Result<(), String> {
        let result = match self.edit_mode {
            EditMode::Global => self.global_handler.handle_command(command),
            EditMode::Position(_) => self.position_handler.handle_command(command),
        };

        match result {
            CommandResult::Ok => Ok(()),
            CommandResult::CommandNotFound => Ok(()),
            CommandResult::Error(error) => Err(error),
            CommandResult::ChangeEditMode(mode) => {
                self.change_edit_mode(mode);
                Ok(())
            }
        }
    }

    fn change_edit_mode(&mut self, mode: EditMode) {
        match mode {
            EditMode::Position(pos) => {
                self.position_handler.change_position(pos.clone());
                self.edit_mode = EditMode::Position(pos.clone());
            }
            _ => self.edit_mode = mode,
        };
    }
}
