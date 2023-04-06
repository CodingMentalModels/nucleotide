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

    pub fn get_name(&self) -> EnemyName {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetType {
    Us,
    Enemy(usize),
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneCommand {
    Damage(u8),
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