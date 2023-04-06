mod game;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::battle::NucleotidePlugin;
use crate::game::ui::UIPlugin;
use crate::game::resources::*;

fn main() {
    App::new()    
        .add_plugins(DefaultPlugins)
        .add_loopless_state(NucleotideState::LoadingUI)
        .add_plugin(UIPlugin)
        .add_plugin(NucleotidePlugin)
        .run();
}