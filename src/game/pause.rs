use bevy::prelude::*;

use crate::game::input::{input_system, PauseUnpauseEvent};
use crate::game::resources::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        let get_pause_states_condition = || {
            in_state(NucleotideState::Paused)
                .or_else(in_state(NucleotideState::Paused))
                .or_else(in_state(NucleotideState::InstantiatingMeta))
                .or_else(in_state(NucleotideState::Drafting))
                .or_else(in_state(NucleotideState::InitializingBattle))
                .or_else(in_state(NucleotideState::CharacterActing))
                .or_else(in_state(NucleotideState::AwaitingBattleInput))
                .or_else(in_state(NucleotideState::StartOfTurn))
                .or_else(in_state(NucleotideState::GeneLoading))
                .or_else(in_state(NucleotideState::GeneCommandHandling))
                .or_else(in_state(NucleotideState::FinishedGeneCommandHandling))
                .or_else(in_state(NucleotideState::EndOfTurn))
                .or_else(in_state(NucleotideState::GeneAnimating))
        };

        app.insert_resource(PausedState(NucleotideState::Paused))
            .add_system(
                pause_system
                    .run_if(get_pause_states_condition())
                    .before(input_system),
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
        if current_state.get() == &NucleotideState::Paused {
            commands.insert_resource(NextState(Some(paused_state.0)));
        } else {
            paused_state.0 = match next_state.0 {
                Some(state) => state,
                None => *current_state.get(),
            };
            commands.insert_resource(NextState(Some(NucleotideState::Paused)));
        }
    }
}

// End Systems
