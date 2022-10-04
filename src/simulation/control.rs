use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;

use crate::simulation::board::BoardSphere;
use crate::simulation::info::SimulationInfo;

#[derive(Debug, Copy, Clone, Default)]
pub struct SimulationStatus {
    pub paused: bool,
    pub ending: bool,
}

pub fn is_simulation_paused(status: Res<SimulationStatus>) -> bool {
    status.paused
}

pub fn is_simulation_paused_or_ending(status: Res<SimulationStatus>) -> bool {
    status.paused || status.ending
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

pub fn wireframe_input_handler(
    kbd: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, Option<&mut Wireframe>), With<BoardSphere>>,
) {
    if kbd.just_pressed(KeyCode::W) {
        let (entity, wireframe) = query.single_mut();
        match wireframe {
            None => commands.entity(entity).insert(Wireframe),
            Some(_) => commands.entity(entity).remove::<Wireframe>(),
        };
    }
}

pub fn auto_pause(mut status: ResMut<SimulationStatus>, info: Res<SimulationInfo>) {
    if info.update_count % 1_000_000 == 0 {
        status.ending = true;
    }
}
