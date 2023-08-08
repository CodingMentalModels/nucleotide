use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Event)]
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
