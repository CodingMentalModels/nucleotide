use bevy::prelude::*;

use crate::game::resources::*;

use super::constants::{STARTING_PLAYER_ENERGY, STARTING_PLAYER_HEALTH};

pub struct MetaPlugin;

impl Plugin for MetaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            instantiate_meta_system.in_schedule(OnEnter(NucleotideState::InstantiatingMeta)),
        ));
    }
}

// Components

// End Components

// Systems

fn instantiate_meta_system(mut commands: Commands, enemy_specs: Res<EnemySpecs>) {
    let player_genome = vec![
        "Sting".to_string(),
        "Repeat Codon".to_string(),
        "Block".to_string(),
        "Reverse Codon".to_string(),
        "Stomp".to_string(),
    ];
    let player = Player::new(
        "Player".to_string(),
        STARTING_PLAYER_HEALTH,
        STARTING_PLAYER_ENERGY,
        player_genome,
    );

    let enemy_queue = EnemyQueue(enemy_specs.get_names());

    commands.insert_resource(player);
    commands.insert_resource(enemy_queue);

    commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
}

// End Systems
