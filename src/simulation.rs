use bevy::prelude::*;

use crate::fixed_timestep::FixedTimestepConfig;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SimulationRunning(pub bool);

pub fn is_simulation_running(flag: Res<SimulationRunning>) -> bool {
    flag.0
}

pub fn simulation_running_input_handler(
    kbd: Res<Input<KeyCode>>,
    mut flag: ResMut<SimulationRunning>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        flag.0 = !flag.0;
    }
}

pub fn simulation_timestep_input_handler(
    kbd: Res<Input<KeyCode>>,
    timestep: Option<ResMut<FixedTimestepConfig>>,
) {
    if let Some(mut timestep) = timestep {
        if kbd.just_pressed(KeyCode::P) {
            timestep.step /= 2;
        }
        if kbd.just_pressed(KeyCode::O) {
            timestep.step *= 2;
        }
    }
}
