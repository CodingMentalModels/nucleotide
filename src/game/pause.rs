
use bevy::{prelude::*};
use iyes_loopless::prelude::*;

use crate::game::resources::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PausedState(NucleotideState::Paused))
            .add_system(pause_input_system);
    }
}

// Components


// End Components

// Systems
fn pause_input_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<CurrentState<NucleotideState>>,
    mut paused_state: ResMut<PausedState>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if current_state.0 == NucleotideState::Paused {
            commands.insert_resource(NextState(paused_state.0));
        } else {
            paused_state.0 = current_state.0;
            commands.insert_resource(NextState(NucleotideState::Paused));
        }
    }
}

// End Systems