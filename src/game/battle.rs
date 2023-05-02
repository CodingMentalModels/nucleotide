use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::game::resources::*;
use crate::game::constants::*;
use crate::game::input::PauseUnpauseEvent;

use super::specs::EnemyName;
use super::specs::{GeneCommand, TargetType};
use super::ui::GenomeDisplayComponent;
use super::ui::{DisplayComponent, CharacterStatComponent};

pub type TargetEntity = Entity;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GeneCommandQueue::default())
        .add_event::<DamageEvent>()
        .add_event::<BlockEvent>()
        .add_event::<GeneProcessingEvent>()
        .add_systems((
            initialize_battle_system.in_schedule(OnEnter(NucleotideState::InitializingBattle)),
            character_acting_system.in_schedule(OnEnter(NucleotideState::CharacterActing)),
            gene_loading_system.in_schedule(OnEnter(NucleotideState::GeneLoading)),
            handle_gene_commands_system.in_schedule(OnEnter(NucleotideState::GeneCommandHandling)),
            update_health_system.in_schedule(OnEnter(NucleotideState::GeneCommandHandling)).after(handle_gene_commands_system),
            update_block_system.in_schedule(OnEnter(NucleotideState::GeneCommandHandling)).after(handle_gene_commands_system),
            update_gene_processing_system.in_schedule(OnEnter(NucleotideState::GeneCommandHandling)).after(handle_gene_commands_system),
            finished_handling_gene_system.run_if(in_state(NucleotideState::GeneEventHandling)),
            render_character_display_system.in_schedule(OnEnter(NucleotideState::GeneAnimating)),
            render_genome_system.in_schedule(OnEnter(NucleotideState::GeneAnimating)),
            finished_animating_gene_system.run_if(in_state(NucleotideState::GeneAnimating)),
        ));
    }
}

// Run Conditions

// End Run Conditions


// Resources

// End Resources


// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DamageEvent(TargetEntity, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlockEvent(TargetEntity, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GeneProcessingEvent(TargetEntity, GeneProcessingEventType);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneProcessingEventType {
    Reverse,
}


// End Events

// Systems

fn initialize_battle_system(
    mut commands: Commands,
    enemy_specs: Res<EnemySpecs>,
    gene_specs: Res<GeneSpecs>,
    player: Res<Player>,
    mut enemy_queue: ResMut<EnemyQueue>,
) {

    let player_entity = instantiate_player(&mut commands, player);
    let enemy_entity = instantiate_enemy(
        &mut commands,
        enemy_queue.pop().expect("There should always be more enemies!"),
        gene_specs,
        enemy_specs,
    );

    commands.insert_resource(CharacterActing(player_entity));
    let character_type_to_entity: Vec<_> = vec![(CharacterType::Player, player_entity), (CharacterType::Enemy, enemy_entity)].into_iter().collect();
    commands.insert_resource(CharacterTypeToEntity(character_type_to_entity));
    commands.insert_resource(NextState(Some(NucleotideState::CharacterActing)));
}

fn character_acting_system(
    mut commands: Commands,
    mut character_acting: ResMut<CharacterActing>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    mut query: Query<(Entity, &mut EnergyComponent)>,
    mut remove_statuses_query: Query<(Entity, &mut BlockComponent)>,
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
) {

    let (
        acting_entity,
        mut energy,
    ) = query.get_mut(character_acting.0).unwrap();

    if energy.energy_remaining == 0 {
        energy.energy_remaining = energy.starting_energy;
        character_acting.0 = character_type_to_entity_map.get_next(acting_entity);

        remove_statuses_query.iter_mut()
            .filter(|(entity, _block)| entity == &character_acting.0)
            .for_each(|(_entity, mut block)| block.0 = 0);
    } else {
        energy.energy_remaining -= 1;
    }

    commands.insert_resource(NextState(Some(NucleotideState::GeneLoading)));
    pause_unpause_event_writer.send(PauseUnpauseEvent);

}

fn gene_loading_system(
    mut commands: Commands,
    character_acting: ResMut<CharacterActing>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    gene_specs: Res<GeneSpecs>,
    mut query: Query<(Entity, &mut GenomeComponent)>,
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
) {

    let (
        acting_entity,
        genome,
    ) = query.get_mut(character_acting.0).unwrap();

    let gene = genome.get_active_gene();

    let gene_spec = gene_specs.0.get_spec_from_name(&gene).expect("Gene should exist as a gene spec.");
    let targets = get_targets(acting_entity, character_type_to_entity_map, gene_spec.get_target());

    gene_command_queue.0.append(
        &mut gene_spec.get_gene_commands().iter()
            .map(|gene_command| targets.iter().map(|target| (gene_command.clone(), target.clone()))).flatten().collect()
    );

    commands.insert_resource(NextState(Some(NucleotideState::GeneCommandHandling)));
    pause_unpause_event_writer.send(PauseUnpauseEvent);

}

fn handle_gene_commands_system(
    mut commands: Commands,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    mut block_event_writer: EventWriter<BlockEvent>,
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
    mut gene_processing_event_writer: EventWriter<GeneProcessingEvent>,
) {

    for (gene_command, target_entity) in gene_command_queue.0.iter() {
        match gene_command {
            GeneCommand::Damage(damage) => {
                damage_event_writer.send(DamageEvent(*target_entity, *damage));
            },
            GeneCommand::Block(amount) => {
                block_event_writer.send(BlockEvent(*target_entity, *amount));
            },
            GeneCommand::ReverseGeneProcessing => {
                gene_processing_event_writer.send(GeneProcessingEvent(*target_entity, GeneProcessingEventType::Reverse));
            }
            _ => panic!("Unimplemented Gene Command!")
        }
    }

    gene_command_queue.0.clear();

    commands.insert_resource(NextState(Some(NucleotideState::GeneEventHandling)));
    pause_unpause_event_writer.send(PauseUnpauseEvent);
}

fn update_health_system(
    mut query: Query<(Entity, &mut HealthComponent, &mut BlockComponent)>,
    mut damage_event_reader: EventReader<DamageEvent>,
) {

    for damage_event in damage_event_reader.iter() {
        if let Ok((_, mut health, mut block)) = query.get_mut(damage_event.0) {
            let damage = damage_event.1.saturating_sub(block.0);
            block.0 = block.0.saturating_sub(damage_event.1);
            health.0 = health.0.saturating_sub(damage);
        }
    }

}

fn update_block_system(
    mut query: Query<(Entity, &mut BlockComponent)>,
    mut block_event_reader: EventReader<BlockEvent>,
) {
    for block_event in block_event_reader.iter() {
        if let Ok((_, mut block)) = query.get_mut(block_event.0) {
            block.0 = block.0.saturating_add(block_event.1);
        }
    }

}

fn update_gene_processing_system(
    mut query: Query<(Entity, &mut GenomeComponent)>,
    mut gene_processing_event_reader: EventReader<GeneProcessingEvent>,
) {

    for gene_processing_event in gene_processing_event_reader.iter() {
        if let Ok((_, mut genome)) = query.get_mut(gene_processing_event.0) {
            match gene_processing_event.1 {
                GeneProcessingEventType::Reverse => {
                    genome.reverse_processing_order();
                }
            }
        }
    }
}

fn finished_handling_gene_system(
    mut commands: Commands,
    character_acting: ResMut<CharacterActing>,
    mut query: Query<&mut GenomeComponent>,
) {
    let mut genome = query.get_mut(character_acting.0).unwrap();

    genome.advance_pointer();

    commands.insert_resource(NextState(Some(NucleotideState::GeneAnimating)));
}

fn render_character_display_system(
    character_display_query: Query<(Entity, &HealthComponent, &BlockComponent, &EnergyComponent)>,
    mut display_query: Query<(&CharacterStatComponent, &mut DisplayComponent)>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
) {

    for (entity, health, block, energy) in character_display_query.iter() {
        for (display_component, mut display) in display_query.iter_mut() {
            if entity == character_type_to_entity_map.get(display_component.0) {
                match display_component.1 {
                    CharacterStatType::Health => display.value = health.0.to_string(),
                    CharacterStatType::Block => display.value = block.0.to_string(),
                    CharacterStatType::Energy => display.value = format!("{} / {}", energy.energy_remaining, energy.starting_energy),
                }
            }
        }
    }
}

fn render_genome_system(
    character_query: Query<(Entity, &GenomeComponent)>,
    mut genome_display_query: Query<(&mut GenomeDisplayComponent)>,
    gene_specs: Res<GeneSpecs>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
) {
    
    for (entity, genome) in character_query.iter() {
        for mut genome_display in genome_display_query.iter_mut() {
            if entity == character_type_to_entity_map.get(genome_display.get_character_type()) {
                let gene_symbol = genome.get_gene(genome_display.get_index()).map( |gene_name| {
                    gene_specs.0.get_symbol_from_name(&gene_name).expect("All genes should have valid symbols.")
                });
                genome_display.maybe_set_gene_symbol(gene_symbol);

                let active_gene_symbol = gene_specs.0.get_symbol_from_name(&genome.get_active_gene())
                    .expect("All genes should have valid symbols.");
                if genome_display.get_gene_symbol() == Some(active_gene_symbol) {
                    genome_display.set_active();
                } else {
                    genome_display.clear_active();
                }
            }
        }
    }

}

fn finished_animating_gene_system(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(NucleotideState::CharacterActing)));
}

// End Systems


// Components
#[derive(Component, Clone, Copy)]
pub struct PlayerComponent;

#[derive(Component, Clone, Copy)]
pub struct EnemyComponent;

#[derive(Component, Clone)]
pub struct GenomeComponent {
    pub genes: Vec<String>,
    pub pointer: usize,
    pub processing_order: GeneProcessingOrder,
}

impl GenomeComponent {

    pub fn new(genes: Vec<String>, pointer: usize, processing_order: GeneProcessingOrder) -> Self {
        GenomeComponent {
            genes,
            pointer,
            processing_order,
        }
    }

    pub fn instantiate(genes: Vec<String>) -> Self {
        Self::new(genes, 0, GeneProcessingOrder::Forward)
    }

    pub fn get_genes(&self) -> Vec<String> {
        self.genes.clone()
    }

    pub fn get_gene(&self, index: usize) -> Option<String> {
        self.genes.get(index).cloned()
    }

    pub fn get_active_gene(&self) -> String {
        self.genes.get(self.pointer).expect(&format!("Genome pointer should always be valid but was {}", self.pointer)).clone()
    }

    pub fn advance_pointer(&mut self) {
        let pointer_delta = match self.processing_order {
            GeneProcessingOrder::Forward => 1,
            GeneProcessingOrder::Reverse => -1,
        };
        self.pointer = ((self.pointer as i32 + pointer_delta).rem_euclid(self.genes.len() as i32)) as usize;
    }

    pub fn reverse_processing_order(&mut self) {
        self.processing_order = match self.processing_order {
            GeneProcessingOrder::Forward => GeneProcessingOrder::Reverse,
            GeneProcessingOrder::Reverse => GeneProcessingOrder::Forward,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct EnergyComponent {
    pub energy_remaining: u8,
    pub starting_energy: u8,
}

impl EnergyComponent {

    pub fn new(energy_remaining: u8, starting_energy: u8) -> Self {
        EnergyComponent {
            energy_remaining,
            starting_energy,
        }
    }

}

#[derive(Component, Clone, Copy)]
pub struct HealthComponent(pub u8);

#[derive(Component, Clone, Copy)]
pub struct BlockComponent(pub u8);


// End Components

// Helper Types

#[derive(Clone)]
pub enum GeneProcessingOrder {
    Forward,
    Reverse,
}

// End Helper Types

// Helper Functions

fn instantiate_player(mut commands: &mut Commands, player: Res<Player>) -> Entity {
    commands.spawn_empty()
        .insert(PlayerComponent)
        .insert(GenomeComponent::instantiate(player.get_genome()))
        .insert(HealthComponent(player.get_health()))
        .insert(BlockComponent(0))
        .insert(EnergyComponent::new(player.get_energy(), player.get_energy()))
        .id()
}

fn instantiate_enemy(commands: &mut Commands, enemy_name: EnemyName, gene_specs: Res<GeneSpecs>, enemy_specs: Res<EnemySpecs>) -> Entity {
    let enemy_spec = enemy_specs.get(enemy_name);
    let genome = enemy_spec.get_genome().iter().map(|s| gene_specs.0.get_spec_from_name(s).expect(&format!("Gene spec not found: {}", s)).get_name().clone()).collect();
    commands.spawn_empty()
        .insert(EnemyComponent)
        .insert(GenomeComponent::instantiate(genome))
        .insert(HealthComponent(enemy_spec.get_health()))
        .insert(BlockComponent(0))
        .insert(EnergyComponent::new(enemy_spec.get_energy(), enemy_spec.get_energy()))
        .id()
}

fn get_targets(acting_entity: Entity, character_type_to_entity_map: Res<CharacterTypeToEntity>, target_type: TargetType) -> Vec<Entity> {
    match target_type {
        TargetType::Us => vec![acting_entity],
        TargetType::RandomEnemy | TargetType::AllEnemies => {
            vec![character_type_to_entity_map.get_next(acting_entity)]
        },
        TargetType::Everyone => {
            character_type_to_entity_map.get_all()
        },
    }
}

// End Helper Functions