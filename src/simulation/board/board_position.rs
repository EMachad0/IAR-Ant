use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Component, Reflect, Deref)]
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
