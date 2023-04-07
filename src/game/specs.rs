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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemySpec {
    name: String,
    health: u8,
    energy: u8,
    gene_pool: Vec<GeneName>,
}

impl EnemySpec {

    pub fn new(name: String, health: u8, energy: u8, gene_pool: Vec<GeneName>) -> Self {
        Self {
            name,
            health,
            energy,
            gene_pool,
        }
    }

    pub fn get_name(&self) -> EnemyName {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetType {
    Us,
    RandomEnemy,
    All,
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
}