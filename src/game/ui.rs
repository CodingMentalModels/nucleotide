use bevy::{prelude::*, asset::LoadState};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::game::resources::*;
use crate::game::constants::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_systems((
                configure_visuals.in_schedule(OnEnter(NucleotideState::LoadingUI)),
                ui_load_system.in_schedule(OnEnter(NucleotideState::LoadingUI)),
                render_system.run_if(in_state(NucleotideState::GeneAnimating)),
                display_state_system,
                display_genome_system,
                paused_ui_system.run_if(in_state(NucleotideState::Paused))
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
        Self {
            prefix,
            value,
        }
    }

    pub fn new_with_u8_value(prefix: String, value: u8) -> Self {
        Self::new(
            prefix,
            value.to_string(),
        )
    }
}


#[derive(Component, Clone)]
pub struct CharacterStatComponent(pub CharacterType, pub CharacterStatType);


#[derive(Component, Clone)]
pub struct GenomeDisplayComponent {
    character_type: CharacterType,
    genes: Vec<Symbol>,
    active_gene: Option<Symbol>,
}

impl GenomeDisplayComponent {

    pub fn new(character_type: CharacterType, genes: Vec<Symbol>) -> Self {
        Self {
            character_type,
            genes,
            active_gene: None,
        }
    }

    pub fn get_character_type(&self) -> CharacterType {
        self.character_type
    }

    pub fn get_genes(&self) -> &Vec<Symbol> {
        &self.genes
    }

    pub fn set_genes(&mut self, genes: Vec<Symbol>) {
        self.genes = genes;
    }

    pub fn get_active_gene(&self) -> Option<Symbol> {
        self.active_gene
    }

    pub fn set_active_gene(&mut self, symbol: Symbol) {
        self.active_gene = Some(symbol);
    }

    pub fn clear_active_gene(&mut self) {
        self.active_gene = None;
    }

}

// End Components

// Systems
fn configure_visuals(mut ctx: EguiContexts) {
    ctx.ctx_mut().set_visuals(
        egui::Visuals {
            window_rounding: 0.0.into(),
            ..Default::default()
        }
    );
}

fn ui_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let font = asset_server.load("fonts/Roboto-Regular.ttf");

    if asset_server.get_load_state(font.clone()) == LoadState::Failed {
        panic!("Failed to load font: {:?}", asset_server.get_load_state(font.clone()));
    }

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            }, background_color: Color::NONE.into(),
            ..Default::default()
        }
    ).with_children(
        |parent| {
            let mut text = ALPHA_LOWER.to_string();
            text.push(BETA_UPPER);
            text.push(GAMMA_UPPER);
            parent.spawn(
                get_text_bundle(
                    &text,
                    get_text_style(font.clone(), Color::WHITE),
                    JustifyContent::Center,
                )
            );
            
            // State
            parent.spawn(
                get_text_bundle(
                    &text,
                    get_text_style(font.clone(), Color::WHITE),
                    JustifyContent::Center,
                )
            ).insert(DisplayComponent::new("State".to_string(), "Uninitialized".to_string()));

            // Player UI
            parent.spawn(
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Percent(0.0),
                            top: Val::Percent(50.0),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    }, background_color: Color::BLACK.into(),
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        get_text_bundle(
                            "Player",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    );
                    parent.spawn(
                        get_text_bundle(
                            "Genes: ",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Genes".to_string(), "XXXX".to_string()))
                    .insert(GenomeDisplayComponent::new(CharacterType::Player, vec![]));
                    parent.spawn(
                        get_text_bundle(
                            "Energy: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Energy".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Player, CharacterStatType::Energy));
                    parent.spawn(
                        get_text_bundle(
                            "Health: 999999",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Health".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Player, CharacterStatType::Health));
                    parent.spawn(
                        get_text_bundle(
                            "Block: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Block".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Player, CharacterStatType::Block));
                }
            );

            // Enemy UI
            parent.spawn(
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(30.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Percent(30.0),
                            top: Val::Percent(0.0),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    }, background_color: Color::BLACK.into(),
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        get_text_bundle(
                            "Enemy",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    );
                    parent.spawn(
                        get_text_bundle(
                            "Energy: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Energy".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Enemy, CharacterStatType::Energy));
                    parent.spawn(
                        get_text_bundle(
                            "Health: 999999",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Health".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Enemy, CharacterStatType::Health));
                    parent.spawn(
                        get_text_bundle(
                            "Block: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new_with_u8_value("Block".to_string(), u8::MAX))
                    .insert(CharacterStatComponent(CharacterType::Enemy, CharacterStatType::Block));
                }
            );
        }
    );

    commands.insert_resource(NextState(Some(NucleotideState::InstantiatingMeta)));

}

fn battle_ui_system(mut egui_ctx: EguiContexts, query: Query<&Text>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Nucleotide");
    });
}

fn render_system(mut query: Query<(&DisplayComponent, &mut Text)>) {

    for (display, mut text) in &mut query {
        text.sections[0].value = format!("{}: {}", display.prefix.to_string(), display.value.to_string());
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
                    text.sections[0].value = format!("{}: {}\nCharacter Acting: {:?}", display.prefix.to_string(), suffix, acting);
                },
                _ => {
                    text.sections[0].value = format!("{}: {}", display.prefix.to_string(), suffix);
                }
            }
        }
    }
}

fn display_genome_system(
    mut query: Query<(&GenomeDisplayComponent, &mut Text)>,
) {
    for (display, mut text) in &mut query {
        text.sections[0].value = display.genes.iter().map(|gene| 
            if display.active_gene == Some(*gene) {
                format!("|{}|", gene)
            } else {
                format!(" {} ", gene.to_string())
            }
        ).collect::<Vec<String>>().join("");
    }
}

fn paused_ui_system(
    mut egui_ctx: EguiContexts,
) {
    egui::Area::new("pause-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(
            egui_ctx.ctx_mut(), 
            |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Paused")
                            .size(20.)
                            .text_style(egui::TextStyle::Heading)
                            .underline()
                            .color(egui::Color32::BLACK)
                        );

                    }
                );
            }
        );
}


// Helper Functions

fn get_text_bundle(
    text: &str,
    text_style: TextStyle,
    justify_content: JustifyContent,
) -> TextBundle {
    TextBundle::from_section(
        text.to_string(),
        text_style
    ).with_text_alignment(TextAlignment::Center)
    .with_style(
        Style {
            align_self: AlignSelf::FlexStart,
            justify_content: justify_content,
            margin: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        }
    )
}

fn get_text_style(font: Handle<Font>, color: Color) -> TextStyle {
    TextStyle {
        font: font,
        font_size: 20.0,
        color: color,
    }
}

// End Helper Functions