use bevy::prelude::*;

use super::fixed_timestep::{FixedTimestepConfig, FixedTimestepInfo};

pub fn timestep_input_handler(
    kbd: Res<Input<KeyCode>>,
    timestep: Option<ResMut<FixedTimestepInfo>>,
    mut commands: Commands,
) {
    if let Some(info) = timestep {
        if kbd.just_pressed(KeyCode::P) {
            commands.insert_resource(FixedTimestepConfig {
                step: Some(info.step / 2),
                ..default()
            });
        }
        if kbd.just_pressed(KeyCode::O) {
            commands.insert_resource(FixedTimestepConfig {
                step: Some(info.step * 2),
                ..default()
            });
        }
    }
}
