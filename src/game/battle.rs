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

fn instantiate_battle_system(commands: Commands) {
    unimplemented!()
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

// End Components