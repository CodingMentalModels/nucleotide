use std::collections::HashMap;

use bevy::{prelude::*};

use crate::game::specs::GeneCommand;

use super::specs::EnemySpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NucleotideState {
    LoadingUI,
    LoadingAssets,
    Menu,
    Paused,
    Drafting,
    InitializingBattle,
    GeneHandling,
    GeneAnimating,
    GameOver,
    Victory,
}


#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GeneCommandQueue {
    pub queue: Vec<GeneCommand>,
}


#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct EnemySpecs(pub HashMap<String, EnemySpec>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GeneSpecs(pub HashMap<String, GeneSpec>);