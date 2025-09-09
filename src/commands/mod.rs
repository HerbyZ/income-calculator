pub mod managers;
pub mod utils;

mod ui;

use chrono::Local;
use managers::{GlobalCommandManager, PositionCommandManager};

use crate::{exit_with_error, Position};

pub enum ChangeEditMode {
    EditPosition(Position),
    PositionChanged(Position),
}

pub enum EditMode {
    Global,
    Position(Position),
}

pub enum CommandResult {
    Ok,
    CommandNotFound,
    ChangeEditMode(ChangeEditMode),
    UpdatePositions(Vec<Position>),
    Error(String),
}

pub struct CommandHandler {
    global_handler: GlobalCommandManager,
    position_handler: Option<PositionCommandManager>,
    positions: Vec<Position>,
    edit_mode: EditMode,
}

const DEFAULT_EDIT_MODE: EditMode = EditMode::Global;

impl CommandHandler {
    pub fn new(initial_positions: &Vec<Position>) -> CommandHandler {
        CommandHandler {
            global_handler: GlobalCommandManager::new(initial_positions),
            position_handler: None,
            positions: initial_positions.to_vec(),
            edit_mode: DEFAULT_EDIT_MODE,
        }
    }

    pub fn show_ui(&self) {
        match self.edit_mode {
            EditMode::Global => {
                self.global_handler.show_ui();
            }
            EditMode::Position(_) => {
                if self.position_handler.is_none() {
                    exit_with_error(String::from("Failed to draw position data"));
                }

                self.position_handler.as_ref().unwrap().show_ui();
            }
        }
    }

    pub fn handle_command(&mut self, input: String) -> Result<(), String> {
        let input_parts = input
            .split(" ")
            .map(|part| part.to_string())
            .collect::<Vec<String>>();

        let command = match input_parts.first() {
            Some(value) => value.to_owned(),
            None => return Err(String::from("No command specified")),
        };

        let arg = match input_parts.len() {
            1 => None,
            2 => input_parts.last(),
            _ => return Err(String::from("Command can accept only 1 argument")),
        };

        let result = match self.edit_mode {
            EditMode::Global => self.global_handler.handle_command(command, arg),
            EditMode::Position(_) => {
                if self.position_handler.is_none() {
                    exit_with_error(String::from("Failed to draw position data"));
                }

                self.position_handler
                    .as_mut()
                    .unwrap()
                    .handle_command(command, arg)
            }
        };

        match result {
            CommandResult::Ok => Ok(()),
            CommandResult::CommandNotFound => Ok(()),
            CommandResult::Error(error) => Err(error),
            CommandResult::UpdatePositions(positions) => {
                self.positions = positions;
                Ok(())
            }
            CommandResult::ChangeEditMode(mode) => {
                self.change_edit_mode(mode);
                Ok(())
            }
        }
    }

    fn change_edit_mode(&mut self, mode: ChangeEditMode) {
        match mode {
            ChangeEditMode::EditPosition(pos) => {
                self.position_handler = Some(PositionCommandManager::new(&pos));
                self.edit_mode = EditMode::Position(pos);
            }
            ChangeEditMode::PositionChanged(position) => {
                let index = self
                    .positions
                    .iter()
                    .position(|pos| pos.id == position.id)
                    .expect("get index of changed position");

                let mut pos = position.clone();
                pos.edited_at = Local::now();

                self.positions[index] = pos;
                self.global_handler = GlobalCommandManager::new(&self.positions);
                self.edit_mode = EditMode::Global;
            }
        };
    }
}
