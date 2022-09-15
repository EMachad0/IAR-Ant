use crate::UpdateTimeDiagnosticsPlugin;
use bevy::app::{App, Plugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod text;
pub mod update_time_diagnostics_plugin;

#[derive(Default)]
pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(UpdateTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::filtered(vec![
                FrameTimeDiagnosticsPlugin::FPS,
                UpdateTimeDiagnosticsPlugin::UPS,
            ]))
            .add_startup_system(text::diagnostics_text_setup)
            .add_system(text::diagnostics_text_update)
            .add_system(text::toggle_diagnostics_text_visibility);
    }
}
