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
    GeneLoading,
    GeneCommandHandling,
    GeneEventHandling,
    GeneAnimating,
    GameOver,
    Victory,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterActing(pub Entity);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GeneCommandQueue(pub Vec<(GeneCommand, Entity)>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySpecs(pub BTreeMap<String, EnemySpec>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneSpecs(pub BTreeMap<String, GeneSpec>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterTypeToEntity(pub Vec<(CharacterType, Entity)>);

impl CharacterTypeToEntity {

    pub fn get(&self, character_type: CharacterType) -> Entity {
        self.0.iter().find(|(ct, _)| *ct == character_type).map(|(_, e)| *e)
            .expect("All character types should be registered when get() is called")
    }

    pub fn get_next(&self, entity: Entity) -> Entity {
        let index = self.0.iter().position(|(_, e)| *e == entity).unwrap();
        let next_index = (index + 1) % self.0.len();
        self.0[next_index].1
    }

    pub fn get_all(&self) -> Vec<Entity> {
        self.0.iter().map(|(_, e)| *e).collect()
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterType {
    Player,
    Enemy,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterStatType {
    Health,
    Block,
    Energy,
}
