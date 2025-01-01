pub mod global;
pub mod position;

use crate::{exit_with_error, Position};
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
    position_handler: Option<PositionHandler>,
    edit_mode: EditMode,
}

const DEFAULT_EDIT_MODE: EditMode = EditMode::Global;

impl CommandHandler {
    pub fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            global_handler: GlobalHandler::new(initial_positions),
            position_handler: None,
            edit_mode: DEFAULT_EDIT_MODE,
        }
    }

    pub fn show_ui(&self) {
        match self.edit_mode {
            EditMode::Global => {
                self.global_handler.drawer.render_positions_table();
                self.global_handler.drawer.draw_help_tooltip();
            }
            EditMode::Position(_) => {
                if self.position_handler.is_none() {
                    exit_with_error(String::from("Failed to draw position data"));
                }

                let drawer = self.position_handler.clone().unwrap().drawer;

                drawer.clone().render_position_info();
                drawer.draw_help_tooltip();
            }
        }
    }

    pub fn handle_command(&mut self, command: String) -> Result<(), String> {
        let result = match self.edit_mode {
            EditMode::Global => self.global_handler.handle_command(command),
            EditMode::Position(_) => {
                if self.position_handler.is_none() {
                    exit_with_error(String::from("Failed to draw position data"));
                }

                self.position_handler
                    .as_mut()
                    .unwrap()
                    .handle_command(command)
            }
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
                self.position_handler = Some(PositionHandler::new(pos.clone()));
                self.edit_mode = EditMode::Position(pos);
            }
            _ => self.edit_mode = mode,
        };
    }
}
