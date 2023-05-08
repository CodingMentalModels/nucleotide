use serde::{Serialize, Deserialize};

pub type GeneName = String;
pub type EnemyName = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GeneSpec {
    name: GeneName,
    text: String,
    target: TargetType,
    gene_commands: Vec<GeneCommand>,
}

impl GeneSpec {

    pub fn new(name: GeneName, text: String, target: TargetType, gene_commands: Vec<GeneCommand>) -> Self {
        Self {
            name,
            text,
            target,
            gene_commands,
        }
    }

    pub fn get_name(&self) -> GeneName {
        self.name.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_target(&self) -> TargetType {
        self.target
    }

    pub fn get_gene_commands(&self) -> Vec<GeneCommand> {
        self.gene_commands.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemySpec {
    name: String,
    health: u8,
    energy: u8,
    genome: Vec<GeneName>,
}

impl EnemySpec {

    pub fn new(name: String, health: u8, energy: u8, genome: Vec<GeneName>) -> Self {
        Self {
            name,
            health,
            energy,
            genome,
        }
    }

    pub fn get_name(&self) -> EnemyName {
        self.name.clone()
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetType {
    Us,
    RandomEnemy,
    AllEnemies,
    Everyone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneCommand {
    Damage(u8),
    Block(u8),
    Heal(u8),
    Status(StatusEffect, u8),
    JumpForwardNGenes(u8),
    ReverseGeneProcessing,
    GainEnergy(u8),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffect {
    Poison,
    Weak,
    RepeatGene,
}

impl StatusEffect {

    pub fn get_activation_timing(&self) -> ActivationTiming {
        match self {
            StatusEffect::Poison => ActivationTiming::EndOfTurn,
            StatusEffect::Weak => ActivationTiming::NotApplicable,
            StatusEffect::RepeatGene => ActivationTiming::EndOfTurn,
        }
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationTiming {
    StartOfTurn,
    EndOfTurn,
    NotApplicable,
}