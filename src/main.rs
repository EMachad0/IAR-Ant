mod diagnostics;
mod simulation;
mod timestep;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use bevy::window::PresentMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::diagnostics::SimulationDiagnosticsPlugin;
use crate::simulation::control::SimulationRunning;
use crate::timestep::fixed_timestep::{FixedTimestepConfig, FixedTimestepStage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub struct FixedUpdateLabel;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Ant!".to_string(),
            width: 800.,
            height: 800.,
            resizable: false,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(SimulationRunning(true))
        .insert_resource(FixedTimestepConfig::new(Duration::from_secs_f64(1. / 60.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(SimulationDiagnosticsPlugin)
        .add_stage_before(
            CoreStage::Update,
            FixedUpdateLabel,
            FixedTimestepStage::empty().with_stage(
                SystemStage::parallel()
                    .with_system(step.run_if(simulation::control::is_simulation_running)),
            ),
        )
        .add_startup_system(add_camera)
        .add_startup_system(setup)
        .add_system(simulation::control::simulation_running_input_handler)
        .add_system(timestep::control::timestep_input_handler)
        .run();
}

#[derive(Component)]
pub struct Actor;

fn setup(mut commands: Commands) {
    // Square
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        })
        .insert(Actor);
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    });
}

pub fn step(mut query: Query<&mut Transform, With<Actor>>, windows: Res<Windows>) {
    let window = windows.get_primary().expect("Could not find a window");
    for mut transform in query.iter_mut() {
        transform.translation += vec3(1.0, 1.0, 0.0);
        transform.translation.x = (transform.translation.x + window.width()) % window.width();
        transform.translation.y = (transform.translation.y + window.height()) % window.height();
    }
}
