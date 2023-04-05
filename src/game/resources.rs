use bevy::{prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NucleotideState {
    Loading,
    Menu,
    Paused,
    Drafting,
    InBattle,
    GameOver,
    Victory,
}