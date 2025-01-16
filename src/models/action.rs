#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Action {
    Long,
    Short,
}

impl Action {
    pub fn from_string(string: String) -> Result<Action, String> {
        match string.to_lowercase().as_str() {
            "l" | "long" | "b" | "buy" => Ok(Action::Long),
            "s" | "short" | "sell" => Ok(Action::Short),
            _ => {
                return Err(format!(
                    "'{}' is not valid position type (long/short)",
                    string
                ))
            }
        }
    }
}
