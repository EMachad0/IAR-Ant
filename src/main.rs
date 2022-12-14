mod camera;
mod consts;
mod diagnostics;
mod inspector;
mod simulation;
mod timestep;

use crate::camera::CameraPlugin;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::window::PresentMode;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::consts::{STARTING_UPS, WINDOW_SIZE};
use crate::diagnostics::SimulationDiagnosticsPlugin;
use crate::inspector::DebugInspectorPlugin;
use crate::simulation::ant::Ant;
use crate::simulation::board::{BoardPosition, IcoBoard};
use crate::simulation::control::SimulationStatus;
use crate::timestep::fixed_timestep::{FixedTimestepConfig, FixedTimestepStage};
use crate::timestep::FixedUpdateLabel;

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Ant!".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            resizable: false,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(SimulationStatus::default())
        .insert_resource(FixedTimestepConfig::new(Duration::from_secs_f64(
            STARTING_UPS,
        )))
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugInspectorPlugin)
        .add_plugin(SimulationDiagnosticsPlugin)
        // Register types
        .register_type::<IcoBoard>()
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
        .add_startup_system_to_stage(StartupStage::PreStartup, simulation::board::icosphere_setup)
        .add_startup_system(simulation::ant::ant_spawn)
        .add_startup_system(simulation::item::item_spawn)
        .add_startup_system(simulation::ant::draw_probability_function)
        // Per Frame Systems
        .add_system(simulation::ant::ant_texture_update)
        .add_system(simulation::ant::ant_position_update)
        .add_system(
            simulation::item::item_pickup_update
                .before(simulation::item::item_position_update),
        )
        .add_system(simulation::item::item_position_update)
        .add_system(simulation::control::simulation_pause_input_handler)
        .add_system(simulation::control::simulation_ending_input_handler)
        .add_system(simulation::control::wireframe_input_handler)
        .add_system(timestep::control::timestep_input_handler)
        // Run
        .run();
}
