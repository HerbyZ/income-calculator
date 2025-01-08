pub mod global;
pub mod position;

use crate::{exit_with_error, Position};
use global::GlobalCommandManager;
use position::PositionCommandManager;

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
    pub fn new(initial_positions: Vec<Position>) -> CommandHandler {
        CommandHandler {
            global_handler: GlobalCommandManager::new(initial_positions.clone()),
            position_handler: None,
            positions: initial_positions,
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
                self.position_handler = Some(PositionCommandManager::new(pos.clone()));
                self.edit_mode = EditMode::Position(pos);
            }
            ChangeEditMode::PositionChanged(position) => {
                let index = self
                    .positions
                    .iter()
                    .position(|pos| pos.id == position.id)
                    .expect("get index of changed position");

                self.positions[index] = position.clone();
                self.global_handler = GlobalCommandManager::new(self.positions.clone());
                self.edit_mode = EditMode::Global;
            }
        };
    }
}
