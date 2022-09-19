use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Copy, Clone, Reflect, FromReflect, Inspectable)]
pub struct Cell {
    pub ant: Option<Entity>,
    pub block: Option<Entity>,
}
