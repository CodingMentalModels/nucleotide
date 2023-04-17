use std::fs;
use std::path::Path;
use bevy::{prelude::*};

use crate::game::resources::*;
use crate::game::constants::*;

use super::specs::EnemySpec;
use super::specs::GeneCommand;
use super::specs::GeneSpec;
use super::specs::TargetType;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                generate_example_specs_system.in_schedule(OnEnter(NucleotideState::LoadingAssets)),
                load_assets_system.in_schedule(OnEnter(NucleotideState::LoadingAssets))
            ));
    }
}

// Systems

fn generate_example_specs_system() {

    let enemy_spec = EnemySpec::new(
        "Example Enemy".to_string(),
        100,
        3,
        vec!["Example Gene 1".to_string(), "Example Gene 2".to_string()],
    );

    let gene_spec = GeneSpec::new(
        "Example Gene 1".to_string(),
        "This is an example gene.".to_string(),
        TargetType::RandomEnemy,
        vec![GeneCommand::Damage(10), GeneCommand::ReverseGeneProcessing],
    );

    let enemy_spec_string = serde_json::to_string(&enemy_spec).expect("Error serializing enemy spec");
    let gene_spec_string = serde_json::to_string(&gene_spec).expect("Error serializing gene spec");

    fs::write(Path::new(ENEMY_SPEC_DIRECTORY).join("example_enemy.json"), enemy_spec_string).expect("Error writing enemy spec");
    fs::write(Path::new(GENE_SPEC_DIRECTORY).join("example_gene.json"), gene_spec_string).expect("Error writing gene spec");
}

fn load_assets_system(mut commands: Commands) {

    let enemies = read_files_from_directory(Path::new(ENEMY_SPEC_DIRECTORY)).into_iter()
        .filter(|s| s.len() > 0)
        .map(|s| serde_json::from_str(&s))
        .collect::<Result<Vec<EnemySpec>, _>>().expect("Error parsing enemy specs");

    let genes = read_files_from_directory(Path::new(GENE_SPEC_DIRECTORY)).into_iter()
        .filter(|s| s.len() > 0)
        .map(|s| serde_json::from_str(&s))
        .collect::<Result<Vec<GeneSpec>, _>>().expect("Error parsing gene specs");

    let enemy_specs = enemies.into_iter().map(|s| (s.get_name().clone(), s)).collect();
    let gene_spec_lookup = GeneSpecLookup::from_specs(genes);

    commands.insert_resource(EnemySpecs(enemy_specs));
    commands.insert_resource(GeneSpecs(gene_spec_lookup));

    commands.insert_resource(NextState(Some(NucleotideState::LoadingUI)));
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