mod game;

use bevy::prelude::*;
use bevy_mod_raycast::DefaultRaycastingPlugin;

use game::assets::AssetsPlugin;
use game::input::InputPlugin;
use game::map::MapPlugin;
use game::meta::MetaPlugin;
use game::pause::PausePlugin;

use crate::game::battle::BattlePlugin;
use crate::game::input::MouseoverRaycastSet;
use crate::game::resources::*;
use crate::game::ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultRaycastingPlugin::<MouseoverRaycastSet>::default(),
        ))
        .add_plugin(AssetsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(MetaPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PausePlugin)
        .add_state::<NucleotideState>()
        .run();
}
