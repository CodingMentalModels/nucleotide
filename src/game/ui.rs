use bevy::{prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::game::resources::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_example_system);
            // .add_enter_system(NucleotideState::Loading, configure_visuals)
            // .add_enter_system(NucleotideState::Loading, ui_load_system);
            // .add_system(ui_system.run_in_state(PongState::InGame))
            // .add_system(paused_ui_system.run_in_state(PongState::Paused))
            // .add_system(paused_input_system.run_in_state(PongState::Paused));
    }
}

// Systems
fn ui_example_system(mut ctx: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(ctx.ctx_mut(), |ui| {
        ui.label("world");
    });
}

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
    commands.spawn_bundle(
        get_text_bundle(
            "Nucleotide",
            get_text_style(font.clone(), Color::WHITE),
            JustifyContent::SpaceBetween,
        )
    );

    commands.insert_resource(NextState(NucleotideState::InBattle));

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