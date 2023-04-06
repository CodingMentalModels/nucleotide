use std::fs;
use std::path::Path;
use bevy::{prelude::*, asset::LoadState};
use iyes_loopless::prelude::*;
use toml::de::Error;

use crate::game::resources::*;
use crate::game::constants::*;

use super::specs::EnemySpec;
use super::specs::GeneSpec;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(NucleotideState::LoadingAssets, load_assets_system);
    }
}

// Systems

fn load_assets_system(mut commands: Commands, mut enemy_specs: ResMut<EnemySpecs>, mut gene_specs: ResMut<GeneSpecs>) {

    let enemies = read_files_from_directory(Path::new(ENEMY_SPEC_DIRECTORY)).into_iter()
        .map(|s| toml::from_str(&s))
        .collect::<Result<Vec<EnemySpec>, Error>>().expect("Error parsing enemy specs");

    let genes = read_files_from_directory(Path::new(GENE_SPEC_DIRECTORY)).into_iter()
        .map(|s| toml::from_str(&s))
        .collect::<Result<Vec<GeneSpec>, Error>>().expect("Error parsing gene specs");

    enemy_specs.0 = enemies.into_iter().map(|s| (s.get_name().clone(), s)).collect();
    gene_specs.0 = genes.into_iter().map(|s| (s.get_name().clone(), s)).collect();

}

// End Systems

// Helper Functions

fn read_files_from_directory(directory: &Path) -> Vec<String> {
    let paths = fs::read_dir(directory);
    let mut to_return = Vec::new();
    match paths {
        Ok(read_dir) => {
            for subpath_result in read_dir {
                match subpath_result {
                    Ok(dir_entry) => {
                        let subpath = dir_entry.path();
                        if (subpath.is_file()) {
                            let contents_result = fs::read_to_string(subpath);
                            match contents_result {
                                Ok(contents) => {
                                    to_return.push(contents);
                                },
                                Err(e) => {
                                    panic!("Error reading file: {}", e);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        panic!("Error reading subpath: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            panic!("Error reading directory: {}", e);
        }
    }

    return to_return;
}

// End Helper Functions