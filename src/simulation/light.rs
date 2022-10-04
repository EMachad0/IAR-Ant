use crate::consts::SUN_DISTANCE;
use crate::diagnostics::simulation_time_diagnostic::SimulationTimeDiagnosticsPlugin;
use bevy::diagnostic::Diagnostics;
use bevy::math::vec3;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_light)
            .add_system(sun_position_update);
    }
}

#[derive(Component)]
pub struct Sun;

pub fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, SUN_DISTANCE, 0.0),
            ..default()
        })
        .insert(Sun);
}

pub fn sun_position_update(
    mut query: Query<&mut Transform, With<Sun>>,
    diagnostics: Res<Diagnostics>,
) {
    let mut transform = query.get_single_mut().expect("Unable to find Sun");
    if let Some(diagnostic) = diagnostics.get(SimulationTimeDiagnosticsPlugin::UPDATE_COUNT) {
        if let Some(value) = diagnostic.value() {
            let angle = value as f32 / 360.0 * (2. * PI);
            transform.translation = vec3(angle.cos(), angle.sin(), 0.0) * SUN_DISTANCE;
        }
    }
}
