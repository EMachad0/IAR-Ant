use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::app::{App, Plugin};
use bevy::ecs::system::{Res, ResMut};
use bevy::prelude::{Deref, DerefMut};
use bevy::time::{Stopwatch, Time};

#[derive(Default, Deref, DerefMut)]
pub struct UpdateTimeStopWatch(Stopwatch);

impl UpdateTimeStopWatch {
    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
}

pub fn update_time_stopwatch_tick(mut stop_watch: ResMut<UpdateTimeStopWatch>, time: Res<Time>) {
    stop_watch.tick(time.delta());
}

/// Adds "update time" diagnostic to an App, specifically "update time", "fps" and "update count"
#[derive(Default)]
pub struct UpdateTimeDiagnosticsPlugin;

pub struct UpdateTimeDiagnosticsState {
    update_count: u64,
}

impl Plugin for UpdateTimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system)
            .insert_resource(UpdateTimeDiagnosticsState { update_count: 0 })
            .insert_resource(UpdateTimeStopWatch::default())
            .add_system(update_time_stopwatch_tick);
    }
}

impl UpdateTimeDiagnosticsPlugin {
    pub const UPS: DiagnosticId = DiagnosticId::from_u128(288146834822086093791974408528866909494);
    pub const UPDATE_COUNT: DiagnosticId =
        DiagnosticId::from_u128(54021991829115352065418785002088010288);
    pub const UPDATE_TIME: DiagnosticId =
        DiagnosticId::from_u128(73441630925388532774622109383099159600);

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::UPDATE_TIME, "update_time", 20).with_suffix("s"));
        diagnostics.add(Diagnostic::new(Self::UPS, "ups", 20));
        diagnostics.add(Diagnostic::new(Self::UPDATE_COUNT, "update_count", 1));
    }

    pub fn diagnostic_system(
        mut diagnostics: ResMut<Diagnostics>,
        mut stopwatch: ResMut<UpdateTimeStopWatch>,
        mut state: ResMut<UpdateTimeDiagnosticsState>,
    ) {
        diagnostics.add_measurement(Self::UPDATE_COUNT, || {
            state.update_count = state.update_count.wrapping_add(1);
            state.update_count as f64
        });

        if stopwatch.elapsed_secs() == 0.0 {
            return;
        }

        diagnostics.add_measurement(Self::UPDATE_TIME, || stopwatch.elapsed_secs_f64());

        diagnostics.add_measurement(Self::UPS, || 1.0 / stopwatch.elapsed_secs_f64());

        stopwatch.reset();
    }
}
