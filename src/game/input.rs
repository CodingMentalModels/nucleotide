use bevy::prelude::*;
use bevy_mod_raycast::{
    print_intersections, DefaultRaycastingPlugin, RaycastMethod, RaycastSource, RaycastSystem,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseUnpauseEvent>()
            .add_systems(
                First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MouseoverRaycastSet>),
            )
            //.add_systems(Update, print_intersections::<MouseoverRaycastSet>)
            .add_systems(Update, input_system);
    }
}

// Components

// End Components

// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
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

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MouseoverRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}
// End Systems

// Structs
#[derive(Debug, Clone, Reflect)]
pub struct MouseoverRaycastSet;

// End Structs
