use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::resources::*;

pub struct NucleotidePlugin;

impl Plugin for NucleotidePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::default())
        .insert_resource(Input::<KeyCode>::default());
        // .add_enter_system(NucleotideState::InBattle, some_system)
        // .add_system(input_system.run_in_state(NucleotideState::InBattle));
    }
}

// Run Conditions

// End Run Conditions


// Resources

// End Resources


// Events

// End Events

// Systems

// End Systems


// Components

// End Components