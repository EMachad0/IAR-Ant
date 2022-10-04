use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

use crate::simulation::info::SimulationInfo;

#[derive(Default)]
pub struct SimulationTimeDiagnosticsPlugin;

impl Plugin for SimulationTimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system);
    }
}

impl SimulationTimeDiagnosticsPlugin {
    pub const UPDATE_COUNT: DiagnosticId =
        DiagnosticId::from_u128(86209885638072597832447592400734);
    pub const ELAPSED_TIME: DiagnosticId =
        DiagnosticId::from_u128(11048120850434880137256329250524);

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::ELAPSED_TIME, "elapsed_time", 1).with_suffix("s"));
        diagnostics.add(Diagnostic::new(Self::UPDATE_COUNT, "update_count", 1));
    }

    pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, info: ResMut<SimulationInfo>) {
        diagnostics.add_measurement(Self::UPDATE_COUNT, || info.update_count as f64);
        diagnostics.add_measurement(Self::ELAPSED_TIME, || info.elapsed_time.as_secs_f64());
    }
}
