mod game;

use bevy::prelude::*;
use game::assets::AssetsPlugin;
use game::input::InputPlugin;
use game::meta::MetaPlugin;
use game::pause::PausePlugin;

use crate::game::battle::BattlePlugin;
use crate::game::ui::UIPlugin;
use crate::game::resources::*;

fn main() {
    App::new()    
        .add_plugins(DefaultPlugins)
        .add_state::<NucleotideState>()
        .add_plugin(AssetsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(MetaPlugin)
        .add_plugin(PausePlugin)
        .run();
}