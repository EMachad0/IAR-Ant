use bevy::prelude::StageLabel;

pub mod control;
mod fixed_timestep;

pub use fixed_timestep::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub struct FixedUpdateLabel;
