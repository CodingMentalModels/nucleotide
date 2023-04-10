use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::resources::*;
use crate::game::constants::*;

use super::specs::{GeneCommand, TargetType};
use super::ui::{DisplayComponent, PlayerHealthDisplayComponent, PlayerBlockDisplayComponent};

pub type TargetEntity = Entity;

pub struct NucleotidePlugin;

impl Plugin for NucleotidePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::default())
        .insert_resource(Input::<KeyCode>::default())
        .insert_resource(GeneCommandQueue::default())
        .add_event::<DamageEvent>()
        .add_event::<BlockEvent>()
        .add_enter_system(NucleotideState::InitializingBattle, instantiate_battle_system)
        .add_enter_system(NucleotideState::CharacterActing, character_acting_system)
        .add_enter_system(NucleotideState::GeneLoading, gene_loading_system)
        .add_enter_system(NucleotideState::GeneCommandHandling, handle_gene_commands_system)
        .add_enter_system(NucleotideState::GeneEventHandling, update_health_system)
        .add_system(finished_handling_gene_system.run_in_state(NucleotideState::GeneEventHandling))
        .add_enter_system(NucleotideState::GeneAnimating, render_health_system)
        .add_system(finished_animating_gene_system.run_in_state(NucleotideState::GeneAnimating));
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

fn instantiate_battle_system(mut commands: Commands, enemy_specs: Res<EnemySpecs>, gene_specs: Res<GeneSpecs>) {

    let player_genome = vec!["Sting".to_string(), "Block".to_string()];
    let player_entity = instantiate_player(&mut commands, player_genome);
    let enemy_entity = instantiate_enemy(&mut commands, enemy_specs, gene_specs, "Enemy");

    commands.insert_resource(CharacterActing(player_entity));
    commands.insert_resource(PlayerEntity(player_entity));
    commands.insert_resource(EnemyEntities(vec![enemy_entity]));

    commands.insert_resource(NextState(NucleotideState::CharacterActing));
}

fn character_acting_system(
    mut commands: Commands,
    mut character_acting: ResMut<CharacterActing>,
    player_entity: Res<PlayerEntity>,
    enemy_entities: Res<EnemyEntities>,
    mut query: Query<(Entity, &mut EnergyComponent)>,
) {

    let (
        acting_entity,
        mut energy,
    ) = query.get_mut(character_acting.0).unwrap();

    if energy.energy_remaining == 0 {
        energy.energy_remaining = energy.starting_energy;
        character_acting.0 = if acting_entity == player_entity.0 {
            enemy_entities.0[0]
        } else {
            player_entity.0
        };
    } else {
        energy.energy_remaining -= 1;
    }

    commands.insert_resource(NextState(NucleotideState::GeneLoading));

}

fn gene_loading_system(
    mut commands: Commands,
    character_acting: ResMut<CharacterActing>,
    player_entity: Res<PlayerEntity>,
    enemy_entities: Res<EnemyEntities>,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    gene_specs: Res<GeneSpecs>,
    query: Query<(Entity, &GenomeComponent, &GenomePointerComponent)>,
) {

    let (
        acting_entity,
        genome,
        genome_pointer
    ) = query.get(character_acting.0).unwrap();

    let gene = &genome.0[genome_pointer.0];
    let gene_spec = gene_specs.0.get(gene).expect("Gene should exist as a gene spec.");
    let targets = get_targets(acting_entity, player_entity.0, &enemy_entities.0, gene_spec.get_target());

    gene_command_queue.0.append(
        &mut gene_spec.get_gene_commands().iter()
            .map(|gene_command| targets.iter().map(|target| (gene_command.clone(), target.clone()))).flatten().collect()
    );

    commands.insert_resource(NextState(NucleotideState::GeneCommandHandling));

}

fn handle_gene_commands_system(
    mut commands: Commands,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    mut block_event_writer: EventWriter<BlockEvent>
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

    commands.insert_resource(NextState(NucleotideState::GeneEventHandling));
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

fn finished_handling_gene_system(mut commands: Commands) {
    commands.insert_resource(NextState(NucleotideState::GeneAnimating));
}

fn render_health_system(
    player_health_query: Query<(&HealthComponent), With<PlayerComponent>>,
    mut display_query: Query<(&mut DisplayComponent), With<PlayerHealthDisplayComponent>>,
) {

    let player_health = player_health_query.iter().next().unwrap();

    let mut display = display_query.iter_mut().next().unwrap();

    display.value = player_health.0;

}

fn finished_animating_gene_system(mut commands: Commands) {
    commands.insert_resource(NextState(NucleotideState::CharacterActing));
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


// End Components

// Helper Functions

fn instantiate_player(mut commands: &mut Commands, genome: Vec<String>) -> Entity {
    commands.spawn()
        .insert(PlayerComponent)
        .insert(GenomeComponent(genome))
        .insert(GenomePointerComponent(0))
        .insert(HealthComponent(STARTING_PLAYER_HEALTH))
        .insert(EnergyComponent::new(STARTING_PLAYER_ENERGY, STARTING_PLAYER_ENERGY))
        .id()
}

fn instantiate_enemy(mut commands: &mut Commands, enemy_specs: Res<EnemySpecs>, gene_specs: Res<GeneSpecs>, enemy_name: &str) -> Entity {
    let enemy_spec = enemy_specs.0.get(enemy_name).expect("Enemy spec not found");
    let genome = enemy_spec.get_genome().iter().map(|s| gene_specs.0.get(s).expect("Gene spec not found").get_name().clone()).collect();
    commands.spawn()
        .insert(EnemyComponent)
        .insert(GenomeComponent(genome))
        .insert(GenomePointerComponent(0))
        .insert(HealthComponent(enemy_spec.get_health()))
        .insert(EnergyComponent::new(enemy_spec.get_energy(), enemy_spec.get_energy()))
        .id()
}

fn get_targets(acting_entity: Entity, player_entity: Entity, enemy_entities: &Vec<Entity>, target_type: TargetType) -> Vec<Entity> {
    match target_type {
        TargetType::Us => vec![acting_entity],
        TargetType::RandomEnemy => {
            if acting_entity == player_entity {
                vec![enemy_entities[0]] // TODO: Make this actually random
            } else {
                vec![player_entity]
            }
        },
        TargetType::AllEnemies => {
            if acting_entity == player_entity {
                enemy_entities.clone()
            } else {
                vec![player_entity]
            }
        }
        TargetType::Everyone => {
            let mut everyone = enemy_entities.clone();
            everyone.push(player_entity);
            everyone
        },
    }
}

// End Helper Functions