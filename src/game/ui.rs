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
                        flex_direction: FlexDirection::ColumnReverse,
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
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
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

    commands.insert_resource(NextState(Some(NucleotideState::InitializingBattle)));

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
) {
    for (display, mut text) in &mut query {
        if display.prefix == "State" {
            text.sections[0].value = format!("{}: {:?}", display.prefix.to_string(), state.0);
        }
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
            align_self: AlignSelf::FlexEnd,
            justify_content: justify_content,
            margin: UiRect::all(Val::Px(25.0)),
            ..Default::default()
        }
    )
}

fn get_text_style(font: Handle<Font>, color: Color) -> TextStyle {
    TextStyle {
        font: font,
        font_size: 30.0,
        color: color,
    }
}

// End Helper Functions