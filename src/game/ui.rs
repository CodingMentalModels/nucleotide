use bevy::{prelude::*, asset::LoadState};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::game::resources::*;
use crate::game::constants::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(NucleotideState::Loading, configure_visuals)
            .add_system(ui_load_system.run_in_state(NucleotideState::Loading))
            .add_system(battle_ui_system.run_in_state(NucleotideState::InBattle));
            // .add_system(paused_ui_system.run_in_state(PongState::Paused))
            // .add_system(paused_input_system.run_in_state(PongState::Paused));
    }
}

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

    if asset_server.get_load_state(font.clone()) == LoadState::Loaded {

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
            }
        );

        commands.insert_resource(NextState(NucleotideState::InBattle));
    }

    if asset_server.get_load_state(font) == LoadState::Failed {
        panic!("Failed to load font.");
    }

}

fn battle_ui_system(mut egui_ctx: ResMut<EguiContext>, query: Query<&Text>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Nucleotide");
    });
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
        font_size: 50.0,
        color: color,
    }
}

// End Helper Functions