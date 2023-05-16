use bevy::ui::RelativeCursorPosition;
use bevy::window::{Cursor, PrimaryWindow};
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::game::constants::*;
use crate::game::resources::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        let get_battle_states_condition = || {
            in_state(NucleotideState::Paused)
                .or_else(in_state(NucleotideState::Paused))
                .or_else(in_state(NucleotideState::InstantiatingMeta))
                .or_else(in_state(NucleotideState::Drafting))
                .or_else(in_state(NucleotideState::InitializingBattle))
                .or_else(in_state(NucleotideState::CharacterActing))
                .or_else(in_state(NucleotideState::StartOfTurn))
                .or_else(in_state(NucleotideState::GeneLoading))
                .or_else(in_state(NucleotideState::GeneCommandHandling))
                .or_else(in_state(NucleotideState::FinishedGeneCommandHandling))
                .or_else(in_state(NucleotideState::EndOfTurn))
                .or_else(in_state(NucleotideState::GeneAnimating))
        };

        app.add_plugin(EguiPlugin).add_systems((
            configure_visuals.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            ui_load_system.in_schedule(OnEnter(NucleotideState::LoadingUI)),
            initialize_battle_ui_system.in_schedule(OnEnter(NucleotideState::InitializingBattle)),
            render_system.run_if(in_state(NucleotideState::GeneAnimating)),
            display_state_system.run_if(get_battle_states_condition()),
            display_genome_system.run_if(get_battle_states_condition()),
            hover_over_gene_system
                .run_if(get_battle_states_condition())
                .after(display_genome_system),
            select_battle_reward_ui_system.run_if(in_state(NucleotideState::SelectBattleReward)),
            paused_ui_system.run_if(in_state(NucleotideState::Paused)),
            game_over_ui_system.run_if(in_state(NucleotideState::GameOver)),
        ));
    }
}

// Components
#[derive(Component, Clone)]
pub struct DisplayComponent {
    pub prefix: String,
    pub value: String,
}

impl DisplayComponent {
    pub fn new(prefix: String, value: String) -> Self {
        Self { prefix, value }
    }

    pub fn new_with_u8_value(prefix: String, value: u8) -> Self {
        Self::new(prefix, value.to_string())
    }
}

#[derive(Component, Clone)]
pub struct CharacterStatComponent(pub CharacterType, pub CharacterStatType);

#[derive(Component, Clone)]
pub struct GenomeDisplayComponent {
    character_type: CharacterType,
    gene: Option<(Symbol, bool)>,
    index: usize,
}

impl GenomeDisplayComponent {
    pub fn new(character_type: CharacterType, gene: Option<(Symbol, bool)>, index: usize) -> Self {
        Self {
            character_type,
            gene,
            index,
        }
    }

    pub fn get_character_type(&self) -> CharacterType {
        self.character_type
    }

    pub fn get_gene_symbol(&self) -> Option<Symbol> {
        self.gene.map(|(symbol, _)| symbol)
    }

    pub fn set_gene_symbol(&mut self, gene: Symbol) {
        self.gene = Some((gene, false));
    }

    pub fn maybe_set_gene_symbol(&mut self, maybe_gene_symbol: Option<Symbol>) {
        match maybe_gene_symbol {
            Some(gene) => self.set_gene_symbol(gene),
            None => self.clear(),
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn is_active(&self) -> bool {
        match self.gene {
            Some((_, is_active)) => is_active,
            None => false,
        }
    }

    pub fn set_active(&mut self) {
        match self.gene {
            Some((symbol, _)) => self.gene = Some((symbol, true)),
            None => (),
        }
    }

    pub fn clear_active(&mut self) {
        match self.gene {
            Some((symbol, _)) => self.gene = Some((symbol, false)),
            None => (),
        }
    }

    pub fn clear(&mut self) {
        self.gene = None;
        self.clear_active();
    }
}

// End Components

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

fn initialize_battle_ui_system(mut commands: Commands, loaded_font: Res<LoadedFont>) {
    let font = loaded_font.0.clone();
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // State
            parent
                .spawn(get_text_bundle(
                    "State: Uninitialized",
                    get_text_style(font.clone(), Color::WHITE, 20.0),
                    JustifyContent::FlexStart,
                    5.0,
                ))
                .insert(DisplayComponent::new(
                    "State".to_string(),
                    "Uninitialized".to_string(),
                ));

            initialize_character_ui(parent, font.clone(), CharacterType::Player, 0.0, 50.0, 50.0);

            initialize_character_ui(parent, font.clone(), CharacterType::Enemy, 30.0, 0.0, 30.0);
        });
}

fn battle_ui_system(mut egui_ctx: EguiContexts, query: Query<&Text>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Nucleotide");
    });
}

fn select_battle_reward_ui_system(mut egui_ctx: EguiContexts, loaded_font: Res<LoadedFont>) {
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

fn render_system(mut query: Query<(&DisplayComponent, &mut Text)>) {
    for (display, mut text) in &mut query {
        text.sections[0].value = format!(
            "{}: {}",
            display.prefix.to_string(),
            display.value.to_string()
        );
    }
}

fn display_state_system(
    mut query: Query<(&mut DisplayComponent, &mut Text)>,
    state: Res<State<NucleotideState>>,
    character_acting: Option<Res<CharacterActing>>,
    character_type_to_entity: Option<Res<CharacterTypeToEntity>>,
    paused_state: Res<PausedState>,
) {
    for (display, mut text) in &mut query {
        if display.prefix == "State" {
            let suffix = if state.0 == NucleotideState::Paused {
                format!("Paused ({:?})", paused_state.0)
            } else {
                format!("{:?}", state.0)
            };
            match (character_acting.as_ref(), character_type_to_entity.as_ref()) {
                (Some(character_acting), Some(character_type_to_entity)) => {
                    let acting = character_type_to_entity.get_character_type(character_acting.0);
                    text.sections[0].value = format!(
                        "{}: {}\nCharacter Acting: {:?}",
                        display.prefix.to_string(),
                        suffix,
                        acting
                    );
                }
                _ => {
                    text.sections[0].value = format!("{}: {}", display.prefix.to_string(), suffix);
                }
            }
        }
    }
}

fn display_genome_system(mut query: Query<(&GenomeDisplayComponent, &mut Text)>) {
    for (display, mut text) in &mut query {
        text.sections[0].value = display.get_gene_symbol().unwrap_or(' ').to_string();
        text.sections[0].style.color = if display.is_active() {
            Color::YELLOW_GREEN
        } else {
            Color::WHITE
        };
    }
}

fn hover_over_gene_system(
    mut commands: Commands,
    mut query: Query<(Entity, &GenomeDisplayComponent, &RelativeCursorPosition)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    gene_specs: Res<GeneSpecs>,
    loaded_font: Res<LoadedFont>,
) {
    assert_eq!(window_query.iter().count(), 1);
    assert_eq!(camera_query.iter().count(), 1);

    for (gene_entity, display, relative_cursor_position) in &mut query {
        commands.entity(gene_entity).despawn_descendants();
        if relative_cursor_position.mouse_over() {
            let gene_card_text = display
                .get_gene_symbol()
                .and_then(|symbol| gene_specs.0.get_card_from_symbol(symbol))
                .unwrap_or(" ".to_string());
            commands.entity(gene_entity).with_children(|parent| {
                render_gene_card(parent, gene_card_text, loaded_font.0.clone());
            });
        }
    }
}

fn paused_ui_system(mut egui_ctx: EguiContexts) {
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

fn game_over_ui_system(mut egui_ctx: EguiContexts) {
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

// Helper Functions

fn get_text_bundle(
    text: &str,
    text_style: TextStyle,
    justify_content: JustifyContent,
    margin_width: f32,
) -> TextBundle {
    TextBundle::from_section(text.to_string(), text_style)
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            justify_content: justify_content,
            margin: UiRect::all(Val::Px(margin_width)),
            ..Default::default()
        })
}

fn get_text_style(font: Handle<Font>, color: Color, font_size: f32) -> TextStyle {
    TextStyle {
        font: font,
        font_size,
        color: color,
    }
}

fn initialize_character_ui(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    character_type: CharacterType,
    left: f32,
    top: f32,
    size: f32,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(size), Val::Percent(size)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(left),
                    top: Val::Percent(top),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(get_text_bundle(
                &character_type.to_string(),
                get_text_style(font.clone(), Color::WHITE, 20.0),
                JustifyContent::FlexStart,
                5.0,
            ));
            initialize_gene_container(parent, font.clone(), character_type);
            initialize_stat_container(
                parent,
                font.clone(),
                character_type,
                CharacterStatType::Energy,
                u8::MAX.to_string(),
            );
            initialize_stat_container(
                parent,
                font.clone(),
                character_type,
                CharacterStatType::Health,
                u8::MAX.to_string(),
            );
            initialize_stat_container(
                parent,
                font.clone(),
                character_type,
                CharacterStatType::Block,
                u8::MAX.to_string(),
            );
            initialize_stat_container(
                parent,
                font.clone(),
                character_type,
                CharacterStatType::Statuses,
                u8::MAX.to_string(),
            );
        });
}

fn initialize_gene_container(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    character_type: CharacterType,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|gene_container| {
            for i in 0..GREEK_ALPHABET.len() {
                gene_container
                    .spawn(get_text_bundle(
                        &GREEK_ALPHABET[i].to_string(),
                        get_text_style(font.clone(), Color::WHITE, 20.0),
                        JustifyContent::FlexStart,
                        5.0,
                    ))
                    .insert(RelativeCursorPosition::default())
                    .insert(GenomeDisplayComponent::new(character_type, None, i));
            }
        });
}

fn initialize_stat_container(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    character_type: CharacterType,
    stat_type: CharacterStatType,
    initial_stat_value: String,
) {
    parent
        .spawn(get_text_bundle(
            "UNINITIALIZED",
            get_text_style(font.clone(), Color::WHITE, 20.0),
            JustifyContent::FlexStart,
            5.0,
        ))
        .insert(DisplayComponent::new(
            stat_type.to_string(),
            initial_stat_value,
        ))
        .insert(CharacterStatComponent(character_type, stat_type));
}

fn render_gene_card(parent: &mut ChildBuilder, display: String, font: Handle<Font>) {
    parent.spawn(
        get_text_bundle(
            &display.to_string(),
            get_text_style(font.clone(), Color::WHITE, 20.0),
            JustifyContent::FlexStart,
            0.0,
        )
        .with_background_color(Color::DARK_GRAY),
    );
}
// End Helper Functions
