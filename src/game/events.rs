#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BattleActionEvent {
    Continue,
    RunAway,
}

impl BattleActionEvent {
    pub fn to_string(&self) -> String {
        match self {
            Self::Continue => "Continue",
            Self::RunAway => "Run Away",
        }
        .to_string()
    }
}
