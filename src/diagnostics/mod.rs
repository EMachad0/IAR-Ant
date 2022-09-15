mod text;

use bevy::app::{App, Plugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::timestep::diagnostic::TimeStepDiagnosticsPlugin;

#[derive(Default)]
pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TimeStepDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::filtered(vec![
                FrameTimeDiagnosticsPlugin::FPS,
                TimeStepDiagnosticsPlugin::SPS,
            ]))
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_startup_system(text::diagnostics_text_setup)
            .add_system(text::diagnostics_text_update)
            .add_system(text::toggle_diagnostics_text_visibility);
    }
}
