#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BattleActionEvent {
    Continue,
}

impl BattleActionEvent {
    pub fn to_string(&self) -> String {
        match self {
            Self::Continue => "Continue",
        }
        .to_string()
    }
}
