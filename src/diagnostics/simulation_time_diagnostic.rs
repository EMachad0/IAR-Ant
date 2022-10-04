use crate::timestep::FixedTimestepInfo;
use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Default)]
pub struct SimulationTimeDiagnosticsPlugin;

#[derive(Default)]
pub struct SimulationTimeDiagnosticsState {
    pub(crate) update_count: u64,
    pub(crate) elapsed_time: Duration,
}

impl Plugin for SimulationTimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system)
            .init_resource::<SimulationTimeDiagnosticsState>();
    }
}

impl SimulationTimeDiagnosticsPlugin {
    pub const UPDATE_COUNT: DiagnosticId =
        DiagnosticId::from_u128(86209885638072597832447592400734);
    pub const ELAPSED_TIME: DiagnosticId =
        DiagnosticId::from_u128(11048120850434880137256329250524);

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::ELAPSED_TIME, "elapsed_time", 20).with_suffix("s"));
        diagnostics.add(Diagnostic::new(Self::UPDATE_COUNT, "update_count", 1));
    }

    pub fn diagnostic_system(
        mut diagnostics: ResMut<Diagnostics>,
        mut state: ResMut<SimulationTimeDiagnosticsState>,
        info: Res<FixedTimestepInfo>,
    ) {
        diagnostics.add_measurement(Self::UPDATE_COUNT, || {
            state.update_count = state.update_count.wrapping_add(1);
            state.update_count as f64
        });
        diagnostics.add_measurement(Self::ELAPSED_TIME, || {
            state.elapsed_time = state.elapsed_time + info.step;
            state.elapsed_time.as_secs_f64()
        });
    }
}
