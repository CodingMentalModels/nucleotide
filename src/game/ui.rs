use bevy::ui::RelativeCursorPosition;
use bevy::window::{Cursor, PrimaryWindow};
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{RichText, Ui};

use crate::game::constants::*;
use crate::game::resources::*;

use super::ui_state::{
    CharacterUIState, GameOverUIState, GenomeUIState, InBattleUIState, PausedUIState,
    RewardUIState, UIState,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_systems((
            configure_visuals.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            ui_load_system.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            render_system.run_if(in_state(NucleotideState::GeneAnimating)),
        ));
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

    commands.insert_resource(UIState::default());
    commands.insert_resource(NextState(Some(NucleotideState::InstantiatingMeta)));
}

fn render_system(ui_state: Res<UIState>, loaded_font: Res<LoadedFont>, mut contexts: EguiContexts) {
    let font = loaded_font.0.clone();

    match *ui_state {
        UIState::Loading => {
            render_loading_ui(contexts);
        }
        UIState::InBattle(in_battle_ui_state) => {
            egui::SidePanel::left("player_character_panel")
                .resizable(false)
                .default_width(200.0)
                .show(contexts.ctx_mut(), |ui| {
                    let player_state = in_battle_ui_state.player_character_state;
                    render_player(ui, player_state, CharacterType::Player);
                });

            egui::SidePanel::right("enemy_character_panel")
                .resizable(false)
                .default_width(200.0)
                .show(contexts.ctx_mut(), |ui| {
                    let enemy_state = in_battle_ui_state.enemy_character_state;
                    render_player(ui, enemy_state, CharacterType::Enemy);
                });
        }
        UIState::Paused(paused_state) => render_paused_ui(contexts),
        UIState::SelectBattleReward(reward_ui_state) => render_select_reward_ui(contexts),
        UIState::GameOver(game_over_state) => render_game_over_ui(contexts),
        _ => {
            panic!("Unhandled UI State: {:?}", ui_state)
        } // Handle other states as needed.
    };
}

// Helper Functions
fn render_select_reward_ui(mut egui_ctx: EguiContexts) {
    egui::Area::new("select-battle-reward-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui_ctx.ctx_mut(), |ui| {
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

fn render_loading_ui(mut egui_ctx: EguiContexts) {
    egui::Area::new("loading-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Loading")
                        .size(20.)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_paused_ui(mut egui_ctx: EguiContexts) {
    egui::Area::new("pause-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui_ctx.ctx_mut(), |ui| {
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

fn render_game_over_ui(mut egui_ctx: EguiContexts) {
    egui::Area::new("game-over-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui_ctx.ctx_mut(), |ui| {
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

fn render_player(
    mut ui: &mut Ui,
    character_state: CharacterUIState,
    character_type: CharacterType,
) {
    let heading = match character_type {
        CharacterType::Player => "Player",
        CharacterType::Enemy => "Enemy",
    };
    ui.vertical_centered_justified(|ui| {
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
                let mut gene_text = RichText::new(gene_state.gene.to_string());
                if gene_state.is_active {
                    gene_text.color(egui::Color32::GREEN);
                }
                let gene_label = ui.label(gene_text);
                if gene_label.hovered() {
                    gene_label.on_hover_text(gene_state.hovertext.clone());
                }
            });
        }
    });
}
// End Helper Functions
