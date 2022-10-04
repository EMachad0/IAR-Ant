use core::time::Duration;
use bevy::prelude::*;

use crate::timestep::FixedTimestepInfo;

#[derive(Debug, Default)]
pub struct SimulationInfo {
    pub update_count: u64,
    pub elapsed_time: Duration,
}

pub fn simulation_info_update(mut state: ResMut<SimulationInfo>, timestep: Res<FixedTimestepInfo>) {
    state.update_count = state.update_count.wrapping_add(1);
    state.elapsed_time = state.elapsed_time + timestep.step;
}