use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{RichText, Ui};

use crate::game::constants::*;
use crate::game::resources::*;

use super::ui_state::InitializingBattleUIState;
use super::ui_state::{
    CharacterUIState, GameOverUIState, GenomeUIState, InBattleUIState, PausedUIState,
    SelectBattleRewardUIState,
};

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
        ));

        app.insert_resource(InitializingBattleUIState::default());
        app.insert_resource(InBattleUIState::from_state(
            NucleotideState::CharacterActing,
        ));
        app.insert_resource(PausedUIState::default());
        app.insert_resource(SelectBattleRewardUIState::default());
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

fn render_battle_system(
    ui_state: Res<InBattleUIState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    loaded_font: Res<LoadedFont>,
    mut contexts: EguiContexts,
) {
    let font = loaded_font.0.clone();
    let window = window_query.single();

    let player_position = egui::Pos2::new(20.0, window.height() - 220.0);
    let player_size = egui::Vec2::new(300.0, 200.0);

    let enemy_position = egui::Pos2::new(window.width() - 220.0, 20.0);
    let enemy_size = egui::Vec2::new(150.0, 100.0);

    egui::containers::Window::new("player-window")
        .movable(false)
        .default_pos(player_position)
        .default_size(player_size)
        .show(contexts.ctx_mut(), |ui| {
            let player_state = ui_state.get_character_state(CharacterType::Player);
            render_player(ui, player_state, CharacterType::Player);
        });

    egui::containers::Window::new("enemy-window")
        .movable(false)
        .default_pos(enemy_position)
        .default_size(enemy_size)
        .show(contexts.ctx_mut(), |ui| {
            let enemy_state = ui_state.get_character_state(CharacterType::Enemy);
            render_player(ui, enemy_state, CharacterType::Enemy);
        });
}

fn render_select_reward_system(
    ui_state: Res<SelectBattleRewardUIState>,
    loaded_font: Res<LoadedFont>,
    mut contexts: EguiContexts,
) {
    egui::Area::new("select-battle-reward-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Select Battle Reward")
                        .size(20.)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
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
    mut ui: &mut Ui,
    character_state: CharacterUIState,
    character_type: CharacterType,
) {
    let heading = match character_type {
        CharacterType::Player => "Player",
        CharacterType::Enemy => "Enemy",
    };
    ui.horizontal(|ui| {
        ui.heading(heading);
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
        for gene_state in &character_state.genome.genes {
            ui.horizontal(|ui| {
                let gene_text = if gene_state.is_active {
                    RichText::new(gene_state.gene.to_string()).color(egui::Color32::GREEN)
                } else {
                    RichText::new(gene_state.gene.to_string())
                };
                let gene_label = ui.label(gene_text);
                if gene_label.hovered() {
                    gene_label.on_hover_text(gene_state.hovertext.clone());
                }
            });
        }
    });
}
// End Helper Functions
