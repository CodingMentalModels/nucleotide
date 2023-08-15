use bevy::prelude::*;

use super::{
    battle::GenomeComponent,
    resources::{CharacterType, GeneSpecLookup, NucleotideState, Player, Symbol},
    specs::StatusEffect,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct InitializingBattleUIState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct InBattleUIState {
    pub nucleotide_state: NucleotideState,
    pub player_character_state: CharacterUIState,
    pub enemy_character_state: CharacterUIState,
}

impl InBattleUIState {
    pub fn new(
        nucleotide_state: NucleotideState,
        player_character_state: CharacterUIState,
        enemy_character_state: CharacterUIState,
    ) -> Self {
        Self {
            nucleotide_state,
            player_character_state,
            enemy_character_state,
        }
    }

    pub fn from_state(state: NucleotideState) -> Self {
        Self::new(
            state,
            CharacterUIState::default(),
            CharacterUIState::default(),
        )
    }

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
            CharacterType::Enemy(name) => self
                .enemy_character_state
                .update_genome(genome, gene_spec_lookup),
        }
    }

    pub fn get_character_state(&self, character_type: &CharacterType) -> CharacterUIState {
        match character_type {
            CharacterType::Player => self.player_character_state.clone(),
            CharacterType::Enemy(_) => self.enemy_character_state.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CharacterUIState {
    pub name: String,
    pub energy_remaining: u8,
    pub total_energy: u8,
    pub health: u8,
    pub block: u8,
    pub status_effects: Vec<(StatusEffect, u8)>,
    pub genome: GenomeUIState,
}

impl CharacterUIState {
    pub fn new(
        name: String,
        energy_remaining: u8,
        total_energy: u8,
        health: u8,
        block: u8,
        status_effects: Vec<(StatusEffect, u8)>,
        genome: GenomeUIState,
    ) -> Self {
        Self {
            name,
            energy_remaining,
            total_energy,
            health,
            block,
            status_effects,
            genome,
        }
    }

    pub fn update_genome(&mut self, genome: &GenomeComponent, gene_spec_lookup: &GeneSpecLookup) {
        self.genome = GenomeUIState::from_genome(genome, gene_spec_lookup);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct PausedUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct SelectBattleRewardUIState(pub Vec<(String, NucleotideState)>);

impl SelectBattleRewardUIState {
    pub fn after_defeating_enemy() -> Self {
        Self::all_options()
    }

    pub fn after_running_away() -> Self {
        let mut to_return = Self::all_options();
        to_return.0.remove(0);
        return to_return;
    }

    fn all_options() -> Self {
        Self(vec![
            (
                "Choose new Gene from Enemy".to_string(),
                NucleotideState::SelectGeneFromEnemy,
            ),
            ("Move a Gene".to_string(), NucleotideState::MoveGene),
            ("Swap two Genes".to_string(), NucleotideState::SwapGenes),
            ("Research a Gene".to_string(), NucleotideState::ResearchGene),
        ])
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct SelectGeneFromEnemyUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub enum MoveGeneUIState {
    #[default]
    FirstSelection,
    SecondSelection(usize),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub enum SwapGenesUIState {
    #[default]
    FirstSelection,
    SecondSelection(usize),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct ResearchGeneUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct GameOverUIState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Resource)]
pub struct VictoryUIState;
