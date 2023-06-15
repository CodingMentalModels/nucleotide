use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{RichText, Ui};

use crate::game::constants::*;
use crate::game::resources::*;

use super::battle::{GenomeComponent, LogState};
use super::ui_state::{
    CharacterUIState, GameOverUIState, GenomeUIState, InBattleUIState, MoveGeneUIState,
    PausedUIState, SelectBattleRewardUIState, SwapGenesUIState, VictoryUIState,
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
            render_game_over_system.run_if(in_state(NucleotideState::GameOver)),
            render_victory_system.run_if(in_state(NucleotideState::Victory)),
            render_select_reward_system.run_if(in_state(NucleotideState::SelectBattleReward)),
            render_select_gene_from_enemy_system
                .run_if(in_state(NucleotideState::SelectGeneFromEnemy)),
            render_move_gene.run_if(in_state(NucleotideState::MoveGene)),
            render_swap_genes.run_if(in_state(NucleotideState::SwapGenes)),
        ));

        app.insert_resource(InitializingBattleUIState::default());
        app.insert_resource(InBattleUIState::from_state(
            NucleotideState::CharacterActing,
        ));
        app.insert_resource(PausedUIState::default());
        app.insert_resource(SelectBattleRewardUIState::default());
        app.insert_resource(SelectGeneFromEnemyUIState::default());
        app.insert_resource(MoveGeneUIState::default());
        app.insert_resource(SwapGenesUIState::default());
        app.insert_resource(GameOverUIState::default());
        app.insert_resource(VictoryUIState::default());
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
    log_state: Res<LogState>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let player_size = egui::Vec2::new(PLAYER_WINDOW_SIZE.0, PLAYER_WINDOW_SIZE.1);
    let enemy_size = egui::Vec2::new(ENEMY_WINDOW_SIZE.0, ENEMY_WINDOW_SIZE.1);

    let enemy_character_type = character_type_to_entity.get_single_enemy();

    let player_state = ui_state.get_character_state(&CharacterType::Player);
    let enemy_state = ui_state.get_character_state(&enemy_character_type);

    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::bottom("log-panel").show(ctx, |mut ui| {
        ui.label(get_underlined_text("Log".to_string()));
        render_log(&mut ui, &log_state);
    });

    egui::TopBottomPanel::top("battle-panel").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                // Render the enemy
                render_character(
                    ui,
                    enemy_state,
                    CharacterType::Enemy(enemy_character_type.to_string()),
                );
            });

            ui.with_layout(
                egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::TopDown,
                    egui::Align::LEFT,
                ),
                |ui| {
                    // Render the player
                    render_character(ui, player_state, CharacterType::Player);
                },
            );
        });
    });
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
    let on_click = |s: usize| match s {
        0 => commands.insert_resource(NextState(Some(NucleotideState::SelectGeneFromEnemy))),
        1 => commands.insert_resource(NextState(Some(NucleotideState::MoveGene))),
        2 => commands.insert_resource(NextState(Some(NucleotideState::SwapGenes))),
        3 => {}
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
    let on_click = |i: usize| {
        assert!(
            i < enemy_genome.len(),
            "The gene is guaranteed to be there."
        );
        let gene = enemy_genome[i].clone();
        player.add_gene(gene.clone());
        commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
    };
    render_options(&mut contexts, heading, options, on_click, Vec::new());
}

fn render_move_gene(
    mut commands: Commands,
    mut ui_state: ResMut<MoveGeneUIState>,
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    let mut genome = player.get_genome();
    match *ui_state {
        MoveGeneUIState::FirstSelection => {
            let on_click = |i: usize| {
                assert!(i < genome.len(), "The gene is guaranteed to be there.");
                *ui_state = MoveGeneUIState::SecondSelection(i)
            };
            render_options(
                &mut contexts,
                "Choose Gene to Move",
                genome.clone(),
                on_click,
                Vec::new(),
            );
        }
        MoveGeneUIState::SecondSelection(first_selection_index) => {
            // This is broken because the string is what gets returned and that's not a gene
            let on_click = |i: usize| {
                player.move_gene(first_selection_index, i);
                commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));
            };
            let mut options = vec!["At the Beginning".to_string()];
            genome.remove(first_selection_index);
            options.append(
                &mut genome
                    .iter()
                    .map(|gene| format!("After {}", gene).to_string())
                    .collect(),
            );
            render_options(
                &mut contexts,
                "Choose location to move the Gene to",
                options,
                on_click,
                Vec::new(),
            );
        }
    }
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
            let on_click = |i: usize| {
                assert!(i < genome.len(), "The gene is guaranteed to be there.");
                *ui_state = SwapGenesUIState::SecondSelection(i)
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
            let on_click = |i: usize| {
                assert!(i < genome.len(), "The gene is guaranteed to be there.");
                player.swap_genes(first_selection_index, i);
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
                        .size(DEFAULT_FONT_SIZE)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_paused_system(ui_state: Res<PausedUIState>, mut contexts: EguiContexts) {
    egui::Area::new("pause-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Paused")
                        .size(DEFAULT_FONT_SIZE)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_game_over_system(ui_state: Res<GameOverUIState>, mut contexts: EguiContexts) {
    egui::Area::new("game-over-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Game Over")
                        .size(DEFAULT_FONT_SIZE)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::BLACK),
                );
            });
        });
}

fn render_victory_system(ui_state: Res<VictoryUIState>, mut contexts: EguiContexts) {
    egui::Area::new("victory-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Victory!")
                        .size(DEFAULT_FONT_SIZE)
                        .text_style(egui::TextStyle::Heading)
                        .underline()
                        .color(egui::Color32::LIGHT_BLUE),
                );
            });
        });
}
// Helper Functions
fn render_character(ui: &mut Ui, character_state: CharacterUIState, character_type: CharacterType) {
    let heading = match character_type {
        CharacterType::Player => format!("{}:", character_type.to_string()),
        CharacterType::Enemy(name) => format!("{}:", name),
    };

    ui.label(get_default_text(heading));
    ui.label(get_default_text(format!(
        "Energy: {}/{}",
        character_state.energy_remaining, character_state.total_energy
    )));
    ui.label(get_default_text(format!(
        "Health: {}",
        character_state.health
    )));
    ui.label(get_default_text(format!(
        "Block: {}",
        character_state.block
    )));
    for (effect, amount) in &character_state.status_effects {
        ui.label(get_default_text(format!(
            "Effect: {:?} x{:?}",
            effect, amount
        )));
    }

    // Display the genome state.
    ui.label(get_default_text("Genome:".to_string()));
    ui.horizontal(|ui| {
        for gene_state in &character_state.genome.genes {
            let gene_text = if gene_state.is_active {
                get_default_text(gene_state.gene.to_string()).color(egui::Color32::GREEN)
            } else {
                get_default_text(gene_state.gene.to_string())
            };
            let gene_label = ui.label(gene_text);
            if gene_label.hovered() {
                gene_label.on_hover_text(get_default_text(gene_state.hovertext.clone()));
            }
        }
    });
}

fn render_log(ui: &mut Ui, log_state: &LogState) {
    let log_window_size = egui::Vec2::new(LOG_WINDOW_SIZE.0, LOG_WINDOW_SIZE.1);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .min_scrolled_height(LOG_WINDOW_SIZE.1)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for log_message in log_state.get_messages().into_iter() {
                ui.label(log_message);
            }
        });
}

fn render_options(
    contexts: &mut EguiContexts,
    heading: &str,
    options: Vec<String>,
    mut on_click: impl FnMut(usize) -> (),
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
                    egui::RichText::new(options[i].clone())
                        .color(egui::Color32::GREEN)
                        .size(DEFAULT_FONT_SIZE)
                } else {
                    egui::RichText::new(options[i].clone()).size(DEFAULT_FONT_SIZE)
                };
                if columns[i]
                    .add(egui::Button::new(text).min_size(button_size.into()))
                    .clicked()
                {
                    on_click(i);
                }
            }
        });
    });
}

fn get_underlined_text(s: String) -> egui::RichText {
    get_default_text(s).underline()
}

fn get_default_text(s: String) -> egui::RichText {
    egui::RichText::new(s)
        .size(DEFAULT_FONT_SIZE)
        .color(egui::Color32::WHITE)
}
// End Helper Functions
