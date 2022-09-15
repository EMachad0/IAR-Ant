use crate::{FixedTimestepConfig, Input, KeyCode, Res, ResMut};

pub fn timestep_input_handler(
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
