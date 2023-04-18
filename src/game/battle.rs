use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::game::resources::*;
use crate::game::constants::*;
use crate::game::input::PauseUnpauseEvent;

use super::specs::EnemyName;
use super::specs::EnemySpec;
use super::specs::{GeneCommand, TargetType};
use super::ui::{DisplayComponent, CharacterStatComponent};

pub type TargetEntity = Entity;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GeneCommandQueue::default())
        .add_event::<DamageEvent>()
        .add_event::<BlockEvent>()
        .add_systems((
            initialize_battle_system.in_schedule(OnEnter(NucleotideState::InitializingBattle)),
            character_acting_system.in_schedule(OnEnter(NucleotideState::CharacterActing)),
            gene_loading_system.in_schedule(OnEnter(NucleotideState::GeneLoading)),
            handle_gene_commands_system.in_schedule(OnEnter(NucleotideState::GeneCommandHandling)),
            update_health_system.in_schedule(OnEnter(NucleotideState::GeneEventHandling)),
            finished_handling_gene_system.run_if(in_state(NucleotideState::GeneEventHandling)),
            render_character_display_system.in_schedule(OnEnter(NucleotideState::GeneAnimating)),
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
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
) {

    let (
        acting_entity,
        mut energy,
    ) = query.get_mut(character_acting.0).unwrap();

    if energy.energy_remaining == 0 {
        energy.energy_remaining = energy.starting_energy;
        character_acting.0 = character_type_to_entity_map.get_next(acting_entity);
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
    query: Query<(Entity, &GenomeComponent, &GenomePointerComponent)>,
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
) {

    let (
        acting_entity,
        genome,
        genome_pointer
    ) = query.get(character_acting.0).unwrap();

    let gene = &genome.0[genome_pointer.0];
    let gene_spec = gene_specs.0.get_spec_from_name(gene).expect("Gene should exist as a gene spec.");
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
) {

    for (gene_command, target_entity) in gene_command_queue.0.iter() {
        match gene_command {
            GeneCommand::Damage(damage) => {
                damage_event_writer.send(DamageEvent(*target_entity, *damage));
            },
            GeneCommand::Block(amount) => {
                block_event_writer.send(BlockEvent(*target_entity, *amount));
            },
            _ => panic!("Unimplemented Gene Command!")
        }
    }

    gene_command_queue.0.clear();

    commands.insert_resource(NextState(Some(NucleotideState::GeneEventHandling)));
    pause_unpause_event_writer.send(PauseUnpauseEvent);
}

fn update_health_system(
    mut query: Query<(Entity, &mut HealthComponent)>,
    mut damage_event_reader: EventReader<DamageEvent>,
    mut block_event_reader: EventReader<BlockEvent>,
) {


    for (entity, mut health) in query.iter_mut() {
        let total_damage = damage_event_reader.iter().filter(|damage_event| damage_event.0 == entity).map(|damage_event| damage_event.1).sum::<u8>();
        let total_block = block_event_reader.iter().filter(|block_event| block_event.0 == entity).map(|block_event| block_event.1).sum::<u8>();

        let final_damage = total_damage.saturating_sub(total_block);
        let final_health = health.0.saturating_sub(final_damage);

        health.0 = final_health;

    }
}

fn finished_handling_gene_system(
    mut commands: Commands,
) {
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
pub struct GenomeComponent(pub Vec<String>);


#[derive(Component, Clone, Copy)]
pub struct GenomePointerComponent(usize);

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

// Helper Functions

fn instantiate_player(mut commands: &mut Commands, player: Res<Player>) -> Entity {
    commands.spawn_empty()
        .insert(PlayerComponent)
        .insert(GenomeComponent(player.get_genome()))
        .insert(GenomePointerComponent(0))
        .insert(HealthComponent(player.get_health()))
        .insert(BlockComponent(0))
        .insert(EnergyComponent::new(player.get_energy(), player.get_energy()))
        .id()
}

fn instantiate_enemy(commands: &mut Commands, enemy_name: EnemyName, gene_specs: Res<GeneSpecs>, enemy_specs: Res<EnemySpecs>) -> Entity {
    let enemy_spec = enemy_specs.get(enemy_name);
    let genome = enemy_spec.get_genome().iter().map(|s| gene_specs.0.get_spec_from_name(s).expect("Gene spec not found").get_name().clone()).collect();
    commands.spawn_empty()
        .insert(EnemyComponent)
        .insert(GenomeComponent(genome))
        .insert(GenomePointerComponent(0))
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