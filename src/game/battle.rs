use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::resources::*;

pub struct NucleotidePlugin;

impl Plugin for NucleotidePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::default())
        .insert_resource(Input::<KeyCode>::default())
        .insert_resource(GeneCommandQueue::default())
        .add_enter_system(NucleotideState::InitializingBattle, instantiate_battle_system)
        .add_enter_system(NucleotideState::CharacterActing, character_acting_system)
        .add_enter_system(NucleotideState::GeneHandling, handle_gene_commands_system)
        .add_system(update_health_system.run_in_state(NucleotideState::GeneHandling))
        .add_system(finished_handling_gene_system.run_in_state(NucleotideState::GeneHandling))
        .add_enter_system(NucleotideState::GeneAnimating, animate_gene_system);
    }
}

// Run Conditions

// End Run Conditions


// Resources

// End Resources


// Events

// End Events

// Systems

fn instantiate_battle_system(mut commands: Commands, enemy_specs: Res<EnemySpecs>, gene_specs: Res<GeneSpecs>) {

    let player_genome = vec!["Sting".to_string(), "Block".to_string()];
    let player_entity = instantiate_player(&mut commands, player_genome);
    let _enemy_entity = instantiate_enemy(&mut commands, enemy_specs, gene_specs, "Enemy");

    commands.insert_resource(CharacterActing(player_entity));

    commands.insert_resource(NextState(NucleotideState::GeneHandling));
}

fn character_acting_system(
    mut commands: Commands,
    character_acting: Res<CharacterActing>,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    gene_specs: Res<GeneSpecs>,
    query: Query<(Entity, &GenomeComponent, &GenomePointerComponent)>,
) {
    let (_entity, genome, genome_pointer) = query.get(character_acting.0).unwrap();

    let gene = &genome.0[genome_pointer.0];

    gene_command_queue.0.append(&mut gene_specs.0.get(gene).expect("Gene should exist as a gene spec.").get_gene_commands());

    commands.insert_resource(NextState(NucleotideState::GeneHandling));

}

fn handle_gene_commands_system(mut gene_command_queue: ResMut<GeneCommandQueue>) {
    unimplemented!()
}

fn update_health_system() {
    unimplemented!()
}

fn finished_handling_gene_system() {
    unimplemented!()
}

fn animate_gene_system() {
    unimplemented!()
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
pub struct HealthComponent(pub u8);


// End Components

// Helper Functions

fn instantiate_player(mut commands: &mut Commands, genome: Vec<String>) -> Entity {
    commands.spawn()
        .insert(PlayerComponent)
        .insert(GenomeComponent(genome))
        .insert(GenomePointerComponent(0))
        .insert(HealthComponent(100))
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
        .id()
}

// End Helper Functions