use std::collections::{BTreeMap};

use bevy::{prelude::*};

use crate::game::specs::GeneCommand;
use crate::game::constants::*;

use super::specs::{EnemySpec, GeneSpec};

pub type Symbol = char;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct PausedState(pub NucleotideState);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States, Resource)]
pub enum NucleotideState {
    #[default]
    LoadingAssets,
    LoadingUI,
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


#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct CharacterActing(pub Entity);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GeneCommandQueue(pub Vec<(GeneCommand, Entity)>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct EnemySpecs(pub BTreeMap<String, EnemySpec>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GeneSpecs(pub GeneSpecLookup);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct CharacterTypeToEntity(pub Vec<(CharacterType, Entity)>);

impl CharacterTypeToEntity {

    pub fn get(&self, character_type: CharacterType) -> Entity {
        self.0.iter().find(|(ct, _)| *ct == character_type).map(|(_, e)| *e)
            .expect("All character types should be registered when get() is called")
    }

    pub fn get_character_type(&self, entity: Entity) -> CharacterType {
        self.0.iter().find(|(_, e)| *e == entity).map(|(ct, _)| *ct)
            .expect("All entities should be registered when get_character_type() is called")
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneSpecLookup {
    name_to_spec: BTreeMap<String, GeneSpec>,
    symbol_to_name: BTreeMap<Symbol, String>,
    name_to_symbol: BTreeMap<String, Symbol>,
}

impl GeneSpecLookup {

    pub fn from_specs(gene_specs: Vec<GeneSpec>) -> Self {
        let name_to_spec = gene_specs.iter().map(|spec| (spec.get_name().clone(), spec.clone())).collect();
        let symbol_to_name: BTreeMap<Symbol, String> = gene_specs.iter().enumerate().map(|(i, spec)| (GREEK_ALPHABET[i], spec.get_name().clone())).collect();
        let name_to_symbol = symbol_to_name.iter().map(|(s, n)| (n.clone(), *s)).collect();
        Self {
            name_to_spec,
            symbol_to_name,
            name_to_symbol,
        }
    }

    pub fn get_spec_from_name(&self, name: &str) -> Option<&GeneSpec> {
        self.name_to_spec.get(name)
    }
    
    pub fn get_spec_from_symbol(&self, symbol: Symbol) -> Option<&GeneSpec> {
        self.symbol_to_name.get(&symbol).and_then(|name| self.name_to_spec.get(name))
    }

    pub fn get_name_from_symbol(&self, symbol: Symbol) -> Option<&String> {
        self.symbol_to_name.get(&symbol)
    }

    pub fn get_symbol_from_name(&self, name: &str) -> Option<Symbol> {
        self.name_to_symbol.get(name).copied()
    }
}