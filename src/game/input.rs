use bevy::{prelude::*};

use crate::game::resources::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PauseUnpauseEvent>()
            .add_system(input_system);
    }
}

// Components


// End Components

// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauseUnpauseEvent;

// End Events

// Systems
pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut pause_unpause_event_writer: EventWriter<PauseUnpauseEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        pause_unpause_event_writer.send(PauseUnpauseEvent);
    }
}

// End Systems