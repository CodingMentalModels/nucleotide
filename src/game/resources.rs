use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::game::constants::*;
use crate::game::specs::GeneCommand;

use super::specs::{EnemyName, EnemySpec, GeneName, GeneSpec};

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
    InstantiatingMeta,
    Drafting,
    InitializingBattle,
    CharacterActing,
    StartOfTurn,
    GeneLoading,
    GeneCommandHandling,
    FinishedGeneCommandHandling,
    EndOfTurn,
    GeneAnimating,
    SelectBattleReward,
    SelectGeneFromEnemy,
    MoveGene,
    SwapGenes,
    ResearchGene,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct EnemyQueue(pub Vec<EnemyName>);

impl EnemyQueue {
    pub fn pop(&mut self) -> Option<EnemyName> {
        self.0.pop()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct Player {
    name: String,
    health: u8,
    energy: u8,
    genome: Vec<GeneName>,
}

impl Player {
    pub fn new(name: String, health: u8, energy: u8, genome: Vec<GeneName>) -> Self {
        Self {
            name,
            health,
            energy,
            genome,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn get_energy(&self) -> u8 {
        self.energy
    }

    pub fn get_genome(&self) -> Vec<GeneName> {
        self.genome.clone()
    }

    pub fn add_gene(&mut self, gene: GeneName) {
        self.genome.push(gene);
    }

    pub fn swap_genes(&mut self, i: usize, j: usize) {
        assert!(i < self.genome.len() && j < self.genome.len());
        self.genome.swap(i, j);
    }

    pub fn move_gene(&mut self, i: usize, j: usize) {
        assert!(i < self.genome.len() && j <= self.genome.len());
        let gene_i = self.genome.remove(i);
        self.genome.insert(j, gene_i);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct CharacterActing(pub Entity);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GeneCommandQueue(pub Vec<(GeneCommand, Entity)>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct EnemySpecs(pub BTreeMap<String, EnemySpec>);

impl EnemySpecs {
    pub fn get(&self, enemy_name: EnemyName) -> &EnemySpec {
        self.0
            .get(&enemy_name.to_string())
            .expect("All enemy names should be registered when get() is called")
    }

    pub fn get_names(&self) -> Vec<EnemyName> {
        self.0.keys().map(|s| s.clone()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GeneSpecs(pub GeneSpecLookup);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct CharacterTypeToEntity(pub Vec<(CharacterType, Entity)>);

impl CharacterTypeToEntity {
    pub fn get(&self, character_type: &CharacterType) -> Entity {
        self.0
            .iter()
            .find(|(ct, _)| ct == character_type)
            .map(|(_, e)| *e)
            .expect("All character types should be registered when get() is called")
    }

    pub fn get_character_type(&self, entity: Entity) -> CharacterType {
        self.0
            .iter()
            .find(|(_, e)| *e == entity)
            .map(|(ct, _)| ct.clone())
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

    pub fn is_player(&self, entity: Entity) -> bool {
        self.get_character_type(entity) == CharacterType::Player
    }

    pub fn is_enemy(&self, entity: Entity) -> bool {
        !self.is_player(entity)
    }

    pub fn get_single_enemy(&self) -> CharacterType {
        let to_return: Vec<CharacterType> = self
            .0
            .iter()
            .filter(|(ct, _)| match ct {
                CharacterType::Enemy(name) => true,
                _ => false,
            })
            .map(|(ct, _)| ct.clone())
            .collect();
        assert_eq!(to_return.len(), 1);
        return to_return.first().unwrap().clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterType {
    Player,
    Enemy(String),
}

impl CharacterType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Player => "Player".to_string(),
            Self::Enemy(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterStatType {
    Health,
    Block,
    Energy,
    Statuses,
}

impl CharacterStatType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Health => "Health",
            Self::Block => "Block",
            Self::Energy => "Energy",
            Self::Statuses => "Statuses",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneSpecLookup {
    name_to_spec: BTreeMap<String, GeneSpec>,
    symbol_to_name: BTreeMap<Symbol, String>,
    name_to_symbol: BTreeMap<String, Symbol>,
}

impl GeneSpecLookup {
    pub fn from_specs(gene_specs: Vec<GeneSpec>) -> Self {
        let name_to_spec = gene_specs
            .iter()
            .map(|spec| (spec.get_name().clone(), spec.clone()))
            .collect();
        let symbol_to_name: BTreeMap<Symbol, String> = gene_specs
            .iter()
            .enumerate()
            .map(|(i, spec)| (GREEK_ALPHABET[i], spec.get_name().clone()))
            .collect();
        let name_to_symbol = symbol_to_name
            .iter()
            .map(|(s, n)| (n.clone(), *s))
            .collect();
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
        self.symbol_to_name
            .get(&symbol)
            .and_then(|name| self.name_to_spec.get(name))
    }

    pub fn get_name_from_symbol(&self, symbol: Symbol) -> Option<&String> {
        self.symbol_to_name.get(&symbol)
    }

    pub fn get_symbol_from_name(&self, name: &str) -> Option<Symbol> {
        self.name_to_symbol.get(name).copied()
    }

    pub fn get_card_from_symbol(&self, symbol: Symbol) -> Option<String> {
        self.get_name_from_symbol(symbol)
            .map(|name| (name, self.get_text_from_symbol(symbol).unwrap()))
            .map(|(name, text)| format!("{} ({}) \n\n{}", name, symbol, text))
    }

    pub fn get_text_from_symbol(&self, symbol: Symbol) -> Option<String> {
        self.get_spec_from_symbol(symbol)
            .map(|spec| spec.get_text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct LoadedFont(pub Handle<Font>);
