use bevy::{prelude::*, asset::LoadState};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::game::resources::*;
use crate::game::constants::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(NucleotideState::LoadingUI, configure_visuals)
            .add_enter_system(NucleotideState::LoadingUI, ui_load_system)
            .add_system(render_system.run_in_state(NucleotideState::GeneAnimating));
            // .add_system(battle_ui_system.run_in_state(NucleotideState::GeneAnimating));
            // .add_system(paused_ui_system.run_in_state(PongState::Paused))
            // .add_system(paused_input_system.run_in_state(PongState::Paused));
    }
}

// Components
#[derive(Component, Clone)]
pub struct DisplayComponent {
    pub prefix: String,
    pub value: u8,
}

impl DisplayComponent {

    pub fn new(prefix: String, value: u8) -> Self {
        Self {
            prefix,
            value,
        }
    }
}


#[derive(Component, Clone)]
pub struct CharacterDisplayComponent(pub CharacterType, pub CharacterDisplayType);

// End Components

// Systems
fn configure_visuals(mut ctx: ResMut<EguiContext>) {
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

    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            }, color: Color::NONE.into(),
            ..Default::default()
        }
    ).with_children(
        |parent| {
            let mut text = ALPHA_LOWER.to_string();
            text.push(BETA_UPPER);
            text.push(GAMMA_UPPER);
            parent.spawn_bundle(
                get_text_bundle(
                    &text,
                    get_text_style(font.clone(), Color::WHITE),
                    JustifyContent::Center,
                )
            );

            // Player UI
            parent.spawn_bundle(
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
                    }, color: Color::BLACK.into(),
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Player",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    );
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Energy: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Energy".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Player, CharacterDisplayType::Energy));
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Health: 999999",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Health".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Player, CharacterDisplayType::Health));
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Block: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Block".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Player, CharacterDisplayType::Block));
                }
            );

            // Enemy UI
            parent.spawn_bundle(
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
                    }, color: Color::BLACK.into(),
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Enemy",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    );
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Energy: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Energy".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Enemy, CharacterDisplayType::Energy));
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Health: 999999",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Health".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Enemy, CharacterDisplayType::Health));
                    parent.spawn_bundle(
                        get_text_bundle(
                            "Block: 0",
                            get_text_style(font.clone(), Color::WHITE),
                            JustifyContent::FlexStart,
                        )
                    ).insert(DisplayComponent::new("Block".to_string(), u8::MAX))
                    .insert(CharacterDisplayComponent(CharacterType::Enemy, CharacterDisplayType::Block));
                }
            );
        }
    );

    commands.insert_resource(NextState(NucleotideState::InitializingBattle));

}

// fn battle_ui_system(mut egui_ctx: ResMut<EguiContext>, query: Query<&Text>) {
//     egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
//         ui.label("Nucleotide");
//     });
// }

fn render_system(mut query: Query<(&DisplayComponent, &mut Text)>) {

    for (display, mut text) in &mut query {
        text.sections[0].value = format!("{}: {}", display.prefix.to_string(), display.value.to_string());
    }

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
    ).with_text_alignment(TextAlignment::TOP_CENTER)
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