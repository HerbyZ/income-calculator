use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Long,
    Short,
}

impl Action {
    pub fn from_string(string: String) -> Result<Action, String> {
        match string.to_lowercase().as_str() {
            "l" | "long" => Ok(Action::Long),
            "s" | "short" => Ok(Action::Short),
            _ => {
                return Err(format!(
                    "'{}' is not valid position type (long/short)",
                    string
                ))
            }
        }
    }
}
