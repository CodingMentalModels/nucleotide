use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Event)]
pub struct ClearCombatFromMapEvent();

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Event)]
pub enum BattleActionEvent {
    Continue,
    ExpressGene,
    RunAway,
    Skip,
}

impl BattleActionEvent {
    pub fn to_string(&self) -> String {
        match self {
            Self::Continue => "Continue",
            Self::ExpressGene => "Express Gene",
            Self::RunAway => "Run Away",
            Self::Skip => "Skip",
        }
        .to_string()
    }

    pub fn all_player_actions() -> Vec<Self> {
        vec![
            BattleActionEvent::ExpressGene,
            BattleActionEvent::Skip,
            BattleActionEvent::RunAway,
        ]
    }
}
