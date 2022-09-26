mod consts;
mod diagnostics;
mod inspector;
mod simulation;
mod timestep;

use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use bevy::window::PresentMode;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::consts::{STARTING_UPS, WINDOW_SIZE};
use crate::diagnostics::SimulationDiagnosticsPlugin;
use crate::inspector::DebugInspectorPlugin;
use crate::simulation::ant::Ant;
use crate::simulation::board::{Board, BoardPosition};
use crate::simulation::control::SimulationStatus;
use crate::timestep::fixed_timestep::{FixedTimestepConfig, FixedTimestepStage};
use crate::timestep::FixedUpdateLabel;

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .insert_resource(WindowDescriptor {
            title: "Ant!".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            resizable: false,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(SimulationStatus::default())
        .insert_resource(FixedTimestepConfig::new(Duration::from_secs_f64(
            STARTING_UPS,
        )))
        .insert_resource(Board::new())
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugInspectorPlugin)
        .add_plugin(SimulationDiagnosticsPlugin)
        // Register types
        .register_type::<Board>()
        .register_type::<Ant>()
        .register_type::<BoardPosition>()
        // Simulation Stage
        .add_stage_before(
            CoreStage::Update,
            FixedUpdateLabel,
            FixedTimestepStage::empty().with_stage(
                SystemStage::parallel().with_system_set(
                    ConditionSet::new()
                        .run_if_not(simulation::control::is_simulation_paused)
                        .with_system(simulation::ant::ant_move)
                        .with_system(simulation::ant::ant_pickup_drop)
                        .into(),
                ),
            ),
        )
        // Setup
        .add_startup_system(add_camera)
        .add_startup_system(simulation::board::board_setup)
        .add_startup_system(simulation::ant::ant_spawn)
        .add_startup_system(simulation::food::food_spawn)
        // Per Frame Systems
        .add_system(simulation::ant::ant_texture_update)
        .add_system(simulation::board::update_removed_board_position.label("remove_board_position"))
        .add_system(simulation::board::update_board_position.after("remove_board_position"))
        .add_system(simulation::control::simulation_pause_input_handler)
        .add_system(simulation::control::simulation_ending_input_handler)
        .add_system(timestep::control::timestep_input_handler)
        // Run
        .run();
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
