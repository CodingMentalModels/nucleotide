use bevy::{prelude::*};

use crate::game::resources::*;
use crate::game::input::{PauseUnpauseEvent, input_system};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PausedState(NucleotideState::Paused))
            .add_system(
                pause_system
                    .before(input_system)
            );
    }
}

// Components


// End Components

// Systems
fn pause_system(
    mut commands: Commands,
    current_state: Res<State<NucleotideState>>,
    next_state: Res<NextState<NucleotideState>>,
    mut pause_unpause_event_reader: EventReader<PauseUnpauseEvent>,
    mut paused_state: ResMut<PausedState>,
) {
    for _ in pause_unpause_event_reader.iter() {
        if current_state.0 == NucleotideState::Paused {
            commands.insert_resource(NextState(Some(paused_state.0)));
        } else {
            paused_state.0 = match next_state.0 {
                Some(state) => state,
                None => current_state.0,
            };
            commands.insert_resource(NextState(Some(NucleotideState::Paused)));
        }
    }
}

// End Systems