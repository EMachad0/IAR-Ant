pub mod simulation_time_diagnostic;
mod text;
pub mod timestep_diagnostic;
pub mod similarity_diagnostic;

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::simulation::control::is_simulation_paused_or_ending;
use crate::timestep::{FixedTimestepStage, FixedUpdateLabel};
use simulation_time_diagnostic::SimulationTimeDiagnosticsPlugin;
use timestep_diagnostic::TimeStepDiagnosticsPlugin;
use similarity_diagnostic::SimilarityDiagnosticsPlugin;

pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TimeStepDiagnosticsPlugin::default())
            .add_plugin(SimulationTimeDiagnosticsPlugin::default())
            .add_plugin(SimilarityDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::filtered(vec![
                // SimulationTimeDiagnosticsPlugin::ELAPSED_TIME,
                // SimulationTimeDiagnosticsPlugin::UPDATE_COUNT,
                // SimilarityDiagnosticsPlugin::SIMILARITY
            ]))
            // .add_plugin(LogDiagnosticsPlugin::default())
            .add_startup_system(text::diagnostics_text_setup)
            .add_system(text::diagnostics_text_update)
            .add_system(text::toggle_diagnostics_text_visibility)
            .stage(FixedUpdateLabel, |stage: &mut FixedTimestepStage| {
                stage
                    .get_system_stage(1)
                    .add_system(TimeStepDiagnosticsPlugin::diagnostic_system)
                    .add_system(
                        SimulationTimeDiagnosticsPlugin::diagnostic_system
                            .run_if_not(is_simulation_paused_or_ending),
                    );
                stage
            });
    }
}
