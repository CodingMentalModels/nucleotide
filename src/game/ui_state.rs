use bevy::prelude::*;

use super::{
    battle::GenomeComponent,
    resources::{CharacterType, GeneSpecLookup, NucleotideState, Symbol},
    specs::StatusEffect,
};

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, Resource)]
pub enum UIState {
    #[default]
    Loading,
    InBattle(InBattleUIState),
    SelectBattleReward(RewardUIState),
    Paused(PausedUIState),
    GameOver(GameOverUIState),
    Victory(VictoryUIState),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InBattleUIState {
    pub nucleotide_state: NucleotideState,
    pub player_character_state: CharacterUIState,
    pub enemy_character_state: CharacterUIState,
}

impl InBattleUIState {
    pub fn update_genome(
        &mut self,
        character_type: CharacterType,
        genome: &GenomeComponent,
        gene_spec_lookup: &GeneSpecLookup,
    ) {
        match character_type {
            CharacterType::Player => self
                .player_character_state
                .update_genome(genome, gene_spec_lookup),
            CharacterType::Enemy => self
                .enemy_character_state
                .update_genome(genome, gene_spec_lookup),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterUIState {
    pub energy_remaining: u8,
    pub total_energy: u8,
    pub health: u8,
    pub block: u8,
    pub status_effects: Vec<(StatusEffect, u8)>,
    pub genome: GenomeUIState,
}

impl CharacterUIState {
    pub fn new(
        energy_remaining: u8,
        total_energy: u8,
        health: u8,
        block: u8,
        status_effects: Vec<(StatusEffect, u8)>,
        genome: GenomeUIState,
    ) -> Self {
        Self {
            energy_remaining,
            total_energy,
            health,
            block,
            status_effects,
            genome,
        }
    }
}

impl CharacterUIState {
    pub fn update_genome(&mut self, genome: &GenomeComponent, gene_spec_lookup: &GeneSpecLookup) {
        self.genome = GenomeUIState::from_genome(genome, gene_spec_lookup);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenomeUIState {
    pub genes: Vec<GeneUIState>,
}

impl GenomeUIState {
    pub fn new(genes: Vec<GeneUIState>) -> Self {
        Self { genes }
    }

    pub fn from_genome(genome: &GenomeComponent, gene_spec_lookup: &GeneSpecLookup) -> Self {
        Self::new(genome.get_gene_ui_states(gene_spec_lookup))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneUIState {
    pub gene: Symbol,
    pub is_active: bool,
    pub hovertext: String,
}

impl GeneUIState {
    pub fn new(gene: Symbol, is_active: bool, hovertext: String) -> Self {
        Self {
            gene,
            is_active,
            hovertext,
        }
    }

    pub fn get_gene_symbol(&self) -> Symbol {
        self.gene
    }

    pub fn set_gene_symbol(&mut self, gene: Symbol) {
        self.gene = gene;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PausedUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RewardUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GameOverUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct VictoryUIState;
