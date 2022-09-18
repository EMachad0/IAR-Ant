use bevy::prelude::StageLabel;

pub mod control;
pub mod diagnostic;
pub mod fixed_timestep;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub struct FixedUpdateLabel;
