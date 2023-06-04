use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{RichText, Ui};

use crate::game::constants::*;
use crate::game::resources::*;

use super::battle::GenomeComponent;
use super::ui_state::{
    CharacterUIState, GameOverUIState, GenomeUIState, InBattleUIState, PausedUIState,
    SelectBattleRewardUIState,
};
use super::ui_state::{InitializingBattleUIState, SelectGeneFromEnemyUIState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        let get_battle_states_condition = || {
            in_state(NucleotideState::Paused)
                .or_else(in_state(NucleotideState::CharacterActing))
                .or_else(in_state(NucleotideState::StartOfTurn))
                .or_else(in_state(NucleotideState::GeneLoading))
                .or_else(in_state(NucleotideState::GeneCommandHandling))
                .or_else(in_state(NucleotideState::FinishedGeneCommandHandling))
                .or_else(in_state(NucleotideState::EndOfTurn))
                .or_else(in_state(NucleotideState::GeneAnimating))
        };

        let or_paused_condition =
            |state: NucleotideState| in_state(NucleotideState::Paused).or_else(in_state(state));

        app.add_plugin(EguiPlugin).add_systems((
            configure_visuals.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            ui_load_system.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            render_initializing_battle_system.run_if(in_state(NucleotideState::InitializingBattle)),
            render_battle_system.run_if(get_battle_states_condition()),
            render_paused_system.run_if(in_state(NucleotideState::Paused)),
            render_select_reward_system.run_if(in_state(NucleotideState::SelectBattleReward)),
            render_select_gene_from_enemy_system
                .run_if(in_state(NucleotideState::SelectGeneFromEnemy)),
        ));

        app.insert_resource(InitializingBattleUIState::default());
        app.insert_resource(InBattleUIState::from_state(
            NucleotideState::CharacterActing,
        ));
        app.insert_resource(PausedUIState::default());
        app.insert_resource(SelectBattleRewardUIState::default());
        app.insert_resource(SelectGeneFromEnemyUIState::default());
        app.insert_resource(GameOverUIState::default());
    }
}

// Systems
fn configure_visuals(mut ctx: EguiContexts) {
    ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn ui_load_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");

    if asset_server.get_load_state(font.clone()) == LoadState::Failed {
        panic!(
            "Failed to load font: {:?}",
            asset_server.get_load_state(font.clone())
        );
    }

    commands.insert_resource(LoadedFont(font.clone()));

    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(NextState(Some(NucleotideState::InstantiatingMeta)));
}

fn render_battle_system(ui_state: Res<InBattleUIState>, mut contexts: EguiContexts) {
    let player_size = egui::Vec2::new(PLAYER_WINDOW_SIZE.0, PLAYER_WINDOW_SIZE.1);
    let enemy_size = egui::Vec2::new(ENEMY_WINDOW_SIZE.0, ENEMY_WINDOW_SIZE.1);

    let player_state = ui_state.get_character_state(CharacterType::Player);
    let enemy_state = ui_state.get_character_state(CharacterType::Enemy);

    render_player(
        &mut contexts,
        player_state,
        CharacterType::Player,
        player_size,
    );
    render_player(&mut contexts, enemy_state, CharacterType::Enemy, enemy_size);
}

fn render_select_reward_system(
    mut commands: Commands,
    ui_state: Res<SelectBattleRewardUIState>,
    mut contexts: EguiContexts,
) {
    let heading = "Select Battle Reward";
    let options = vec![
        "Choose new Gene from Enemy".to_string(),
        "Move a Gene".to_string(),
        "Swap two Genes".to_string(),
        "Research a Gene".to_string(),
    ];
    let on_click = |s: &str| match s {
        "Choose new Gene from Enemy" => {
            commands.insert_resource(NextState(Some(NucleotideState::SelectGeneFromEnemy)))
        }
        "Move a Gene" => commands.insert_resource(NextState(Some(NucleotideState::MoveGene))),
        "Swap two Genes" => commands.insert_resource(NextState(Some(NucleotideState::SwapGene))),
        "Research a Gene" => {
            commands.insert_resource(NextState(Some(NucleotideState::ResearchGene)))
        }
        v => panic!("Bad value: {}", v),
    };
    render_options(&mut contexts, heading, options, on_click);
}

fn render_select_gene_from_enemy_system(
    mut commands: Commands,
    ui_state: Res<SelectGeneFromEnemyUIState>,
    mut contexts: EguiContexts,
    mut genome_query: Query<(Entity, &mut GenomeComponent)>,
    mut player: ResMut<Player>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let heading = "Select Gene from Enemy";
    let (maybe_player_genome, maybe_enemy_genome) = genome_query
        .iter_mut()
        .map(|(entity, genome)| {
            if character_type_to_entity.is_player(entity) {
                (Some(genome), None)
            } else if character_type_to_entity.is_enemy(entity) {
                (None, Some(genome))
            } else {
                (None, None)
            }
        })
        .fold((None, None), |mut acc, e| {
            if e.0.is_some() {
                assert!(acc.0.is_none());
                acc.0 = e.0;
            }
            if e.1.is_some() {
                assert!(acc.1.is_none());
                acc.1 = e.1;
            }
            acc
        });

    let mut player_genome = maybe_player_genome.expect("Player genome should exist.");
    let enemy_genome = maybe_enemy_genome.expect("Enemy genome should exist.");

    let options = enemy_genome.get_genes();
    let options_clone = options.clone();
    let on_click = |s: &str| {
        let gene = options_clone
            .iter()
            .filter(|gene| gene.as_str() == s)
            .next()
            .expect("The gene is guaranteed to be there.");
        player_genome.add_gene(gene.clone());
        player.add_gene(gene.clone());
        commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
    };
    render_options(&mut contexts, heading, options, on_click);
}

fn render_initializing_battle_system(
    ui_state: Res<InitializingBattleUIState>,
    loaded_font: Res<LoadedFont>,
    mut contexts: EguiContexts,
) {
    egui::Area::new("initiazing-battle-screen")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Initializing Battle")
                        .size(20.)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_paused_system(
    ui_state: Res<PausedUIState>,
    loaded_font: Res<LoadedFont>,
    mut contexts: EguiContexts,
) {
    egui::Area::new("pause-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Paused")
                        .size(20.)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_game_over_system(
    ui_state: Res<GameOverUIState>,
    loaded_font: Res<LoadedFont>,
    mut contexts: EguiContexts,
) {
    egui::Area::new("game-over-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Game Over")
                        .size(20.)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

// Helper Functions
fn render_player(
    contexts: &mut EguiContexts,
    character_state: CharacterUIState,
    character_type: CharacterType,
    size: egui::Vec2,
) {
    let (window_name, heading, anchor, offset) = match character_type {
        CharacterType::Player => (
            "player-window",
            "Player:",
            egui::Align2::LEFT_BOTTOM,
            egui::Vec2::new(CHARACTER_WINDOW_OFFSET, -CHARACTER_WINDOW_OFFSET),
        ),
        CharacterType::Enemy => (
            "enemy-window",
            "Enemy:",
            egui::Align2::RIGHT_TOP,
            egui::Vec2::new(-CHARACTER_WINDOW_OFFSET, CHARACTER_WINDOW_OFFSET),
        ),
    };

    egui::containers::Window::new(window_name)
        .movable(false)
        .title_bar(false)
        .anchor(anchor, offset)
        .default_size(size)
        .fixed_size(size)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(heading);
            ui.label(format!(
                "Energy: {}/{}",
                character_state.energy_remaining, character_state.total_energy
            ));
            ui.label(format!("Health: {}", character_state.health));
            ui.label(format!("Block: {}", character_state.block));
            for (effect, amount) in &character_state.status_effects {
                ui.label(format!("Effect: {:?} x{:?}", effect, amount));
            }

            // Display the genome state.
            ui.label("Genome:");
            ui.horizontal(|ui| {
                for gene_state in &character_state.genome.genes {
                    let gene_text = if gene_state.is_active {
                        RichText::new(gene_state.gene.to_string()).color(egui::Color32::GREEN)
                    } else {
                        RichText::new(gene_state.gene.to_string())
                    };
                    let gene_label = ui.label(gene_text);
                    if gene_label.hovered() {
                        gene_label.on_hover_text(gene_state.hovertext.clone());
                    }
                }
            });
        });
}

fn render_options(
    contexts: &mut EguiContexts,
    heading: &str,
    options: Vec<String>,
    mut on_click: impl FnMut(&str) -> (),
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading(heading);
        ui.separator();

        let n_columns = options.len();
        let button_size = egui::Vec2::new(OPTION_CARD_SIZE.0, OPTION_CARD_SIZE.1);
        ui.columns(n_columns, |columns| {
            for i in 0..n_columns {
                if columns[i]
                    .add(egui::Button::new(options[i].clone()).min_size(button_size.into()))
                    .clicked()
                {
                    on_click(options[i].as_str());
                }
            }
        });
    });
}
// End Helper Functions
