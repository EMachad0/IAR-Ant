use bevy::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SimulationTimer(Timer);

impl SimulationTimer {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, true))
    }
}

pub fn on_simulation_timer(timer: Res<SimulationTimer>) -> bool {
    timer.just_finished()
}

pub fn simulation_tick(mut simulation_state: ResMut<SimulationTimer>, time: Res<Time>) {
    simulation_state.tick(time.delta());
}

pub fn simulation_control(kbd: Res<Input<KeyCode>>, mut timer: ResMut<SimulationTimer>) {
    if kbd.just_pressed(KeyCode::Space) {
        if timer.paused() {
            timer.unpause();
        } else {
            timer.pause()
        }
    }
    if kbd.just_pressed(KeyCode::P) {
        let duration = timer.duration() / 2;
        timer.set_duration(duration);
    }
    if kbd.just_pressed(KeyCode::O) {
        let duration = timer.duration() * 2;
        if duration > Duration::ZERO {
            timer.set_duration(duration);
        }
    }
}
