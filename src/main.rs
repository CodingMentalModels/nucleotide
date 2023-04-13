mod game;

use bevy::prelude::*;
use game::assets::AssetsPlugin;
use game::pause::PausePlugin;

use crate::game::battle::NucleotidePlugin;
use crate::game::ui::UIPlugin;
use crate::game::resources::*;

fn main() {
    App::new()    
        .add_plugins(DefaultPlugins)
        .add_state::<NucleotideState>()
        .add_plugin(AssetsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(NucleotidePlugin)
        .add_plugin(PausePlugin)
        .run();
}