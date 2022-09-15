use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::fixed_timestep::TimeStepDiagnosticsPlugin;

#[derive(Component)]
pub struct DiagnosticsText;

pub fn diagnostics_text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(DiagnosticsText)
        .insert_bundle(TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLUE,
                }),
                TextSection::new(
                    "\n",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
                TextSection::new(
                    "UPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLUE,
                }),
            ]),
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            visibility: Visibility {
                is_visible: cfg!(debug_assertions),
            },
            ..default()
        });
}

pub fn diagnostics_text_update(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    for mut text in &mut query {
        if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = diagnostic.average() {
                text.sections[1].value = format!("{average:.0}");
            }
        }
        if let Some(diagnostic) = diagnostics.get(TimeStepDiagnosticsPlugin::SPS) {
            if let Some(average) = diagnostic.average() {
                text.sections[4].value = format!("{average:.0}");
            }
        }
    }
}

pub fn toggle_diagnostics_text_visibility(
    kbd: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<DiagnosticsText>>,
) {
    if kbd.just_pressed(KeyCode::F3) {
        for mut visibility in &mut query {
            let current = visibility.is_visible;
            visibility.is_visible = !current;
        }
    }
}
