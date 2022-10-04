mod text;
pub mod timestep_diagnostic;

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use crate::timestep::{FixedTimestepStage, FixedUpdateLabel};
use timestep_diagnostic::TimeStepDiagnosticsPlugin;

pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TimeStepDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::filtered(vec![
            //     FrameTimeDiagnosticsPlugin::FPS,
            //     TimeStepDiagnosticsPlugin::SPS,
            // ]))
            // .add_plugin(LogDiagnosticsPlugin::default())
            .add_startup_system(text::diagnostics_text_setup)
            .add_system(text::diagnostics_text_update)
            .add_system(text::toggle_diagnostics_text_visibility)
            .stage(FixedUpdateLabel, |stage: &mut FixedTimestepStage| {
                stage
                    .get_system_stage(1)
                    .add_system(TimeStepDiagnosticsPlugin::diagnostic_system);
                stage
            });
    }
}
