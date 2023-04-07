use std::collections::{BTreeMap};

use bevy::{prelude::*};

use crate::game::specs::GeneCommand;

use super::specs::{EnemySpec, GeneSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NucleotideState {
    LoadingUI,
    LoadingAssets,
    Menu,
    Paused,
    Drafting,
    InitializingBattle,
    CharacterActing,
    GeneHandling,
    GeneAnimating,
    GameOver,
    Victory,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterActing(pub Entity);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GeneCommandQueue(pub Vec<GeneCommand>);


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySpecs(pub BTreeMap<String, EnemySpec>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneSpecs(pub BTreeMap<String, GeneSpec>);