use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct SimulationStatus {
    pub paused: bool,
    pub ending: bool,
}

pub fn is_simulation_paused(status: Res<SimulationStatus>) -> bool {
    status.paused
}

pub fn simulation_pause_input_handler(
    kbd: Res<Input<KeyCode>>,
    mut status: ResMut<SimulationStatus>,
) {
    if kbd.just_pressed(KeyCode::K) {
        status.paused = !status.paused;
    }
}

pub fn simulation_ending_input_handler(
    kbd: Res<Input<KeyCode>>,
    mut status: ResMut<SimulationStatus>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        status.ending = !status.ending;
    }
}
