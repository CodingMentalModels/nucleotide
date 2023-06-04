use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{RichText, Ui};

use crate::game::constants::*;
use crate::game::resources::*;

use super::battle::GenomeComponent;
use super::ui_state::{
    CharacterUIState, GameOverUIState, GenomeUIState, InBattleUIState, PausedUIState,
    SelectBattleRewardUIState, SwapGenesUIState,
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
            render_swap_genes.run_if(in_state(NucleotideState::SwapGenes)),
        ));

        app.insert_resource(InitializingBattleUIState::default());
        app.insert_resource(InBattleUIState::from_state(
            NucleotideState::CharacterActing,
        ));
        app.insert_resource(PausedUIState::default());
        app.insert_resource(SelectBattleRewardUIState::default());
        app.insert_resource(SelectGeneFromEnemyUIState::default());
        app.insert_resource(SwapGenesUIState::default());
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
    mut contexts: EguiContexts,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let player_size = egui::Vec2::new(PLAYER_WINDOW_SIZE.0, PLAYER_WINDOW_SIZE.1);
    let enemy_size = egui::Vec2::new(ENEMY_WINDOW_SIZE.0, ENEMY_WINDOW_SIZE.1);

    let enemy_character_type = character_type_to_entity.get_single_enemy();

    let player_state = ui_state.get_character_state(&CharacterType::Player);
    let enemy_state = ui_state.get_character_state(&enemy_character_type);

    render_character(
        &mut contexts,
        player_state,
        CharacterType::Player,
        player_size,
    );
    render_character(&mut contexts, enemy_state, enemy_character_type, enemy_size);
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
        "Swap two Genes" => commands.insert_resource(NextState(Some(NucleotideState::SwapGenes))),
        "Research a Gene" => {
            commands.insert_resource(NextState(Some(NucleotideState::ResearchGene)))
        }
        v => panic!("Bad value: {}", v),
    };
    render_options(&mut contexts, heading, options, on_click, Vec::new());
}

fn render_select_gene_from_enemy_system(
    mut commands: Commands,
    ui_state: Res<SelectGeneFromEnemyUIState>,
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    enemy_specs: Res<EnemySpecs>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let heading = "Select Gene from Enemy";

    let enemy_character_type = character_type_to_entity.get_single_enemy();
    let enemy_name = enemy_character_type.to_string();
    let enemy_genome = enemy_specs.get(enemy_name).get_genome();

    let options = enemy_genome.clone();
    let on_click = |s: &str| {
        let gene = enemy_genome
            .iter()
            .filter(|gene| gene.as_str() == s)
            .next()
            .expect("The gene is guaranteed to be there.");
        player.add_gene(gene.clone());
        commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
    };
    render_options(&mut contexts, heading, options, on_click, Vec::new());
}

fn render_swap_genes(
    mut commands: Commands,
    mut ui_state: ResMut<SwapGenesUIState>,
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let genome = player.get_genome();
    match *ui_state {
        SwapGenesUIState::FirstSelection => {
            let on_click = |s: &str| {
                let gene_index = genome
                    .iter()
                    .enumerate()
                    .filter(|(i, gene)| gene.as_str() == s)
                    .map(|(i, _)| i)
                    .next()
                    .expect("The gene is guaranteed to be there.");
                *ui_state = SwapGenesUIState::SecondSelection(gene_index)
            };
            render_options(
                &mut contexts,
                "Choose First Gene to Swap",
                genome.clone(),
                on_click,
                Vec::new(),
            );
        }
        SwapGenesUIState::SecondSelection(first_selection_index) => {
            let on_click = |s: &str| {
                let second_selection_index = genome
                    .iter()
                    .enumerate()
                    .filter(|(_, gene)| gene.as_str() == s)
                    .map(|(i, _)| i)
                    .next()
                    .expect("The gene is guaranteed to be there.");
                player.swap_genes(first_selection_index, second_selection_index);
                commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
            };
            render_options(
                &mut contexts,
                "Choose Second Gene to Swap",
                genome.clone(),
                on_click,
                vec![first_selection_index],
            );
        }
    }
}

fn render_initializing_battle_system(
    ui_state: Res<InitializingBattleUIState>,
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
fn render_character(
    contexts: &mut EguiContexts,
    character_state: CharacterUIState,
    character_type: CharacterType,
    size: egui::Vec2,
) {
    let (window_name, heading, anchor, offset) = match character_type {
        CharacterType::Player => (
            "player-window",
            format!("{}:", character_type.to_string()),
            egui::Align2::LEFT_BOTTOM,
            egui::Vec2::new(CHARACTER_WINDOW_OFFSET, -CHARACTER_WINDOW_OFFSET),
        ),
        CharacterType::Enemy(name) => (
            "enemy-window",
            format!("{}:", name),
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
    highlighted_options: Vec<usize>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading(heading);
        ui.separator();

        let n_columns = options.len();
        let button_size = egui::Vec2::new(OPTION_CARD_SIZE.0, OPTION_CARD_SIZE.1);
        ui.columns(n_columns, |columns| {
            for i in 0..n_columns {
                let text = if highlighted_options.contains(&i) {
                    egui::RichText::new(options[i].clone()).color(egui::Color32::GREEN)
                } else {
                    egui::RichText::new(options[i].clone())
                };
                if columns[i]
                    .add(egui::Button::new(text).min_size(button_size.into()))
                    .clicked()
                {
                    on_click(options[i].as_str());
                }
            }
        });
    });
}
// End Helper Functions
