use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Debug, Copy, Clone, Component, Deref, Reflect, FromReflect, Inspectable)]
#[reflect(Component)]
pub struct BoardPosition(usize);

impl BoardPosition {
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub fn idx(&self) -> usize {
        self.0
    }
}
