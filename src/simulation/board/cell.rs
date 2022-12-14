use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Copy, Clone, Reflect, FromReflect, Inspectable)]
pub struct Cell {
    pub food: Option<Entity>,
}
