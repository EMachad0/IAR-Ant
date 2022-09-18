use crate::{BOARD_SIZE, CELL_SIZE};
use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Component)]
pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl BoardPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn to_world_position(&self) -> (f32, f32) {
        let x_off = (self.x as f32) * CELL_SIZE;
        let y_off = (self.y as f32) * CELL_SIZE;
        let x = x_off + CELL_SIZE / 2.;
        let y = y_off + CELL_SIZE / 2.;
        (x, y)
    }

    pub fn add(&mut self, dx: i32, dy: i32) -> &mut Self {
        self.x = (self.x as i32 + dx).clamp(0, BOARD_SIZE as i32) as u32;
        self.y = (self.y as i32 + dy).clamp(0, BOARD_SIZE as i32) as u32;
        self
    }
}

impl From<BoardPosition> for Vec2 {
    fn from(pos: BoardPosition) -> Self {
        pos.to_world_position().into()
    }
}

impl From<BoardPosition> for Vec3 {
    fn from(pos: BoardPosition) -> Self {
        Self::from((pos.into(), 0.0))
    }
}

impl From<BoardPosition> for Transform {
    fn from(pos: BoardPosition) -> Self {
        Self::from_translation(pos.into())
    }
}

pub fn update_board_position(
    mut query: Query<(&mut Transform, &BoardPosition), Changed<BoardPosition>>,
) {
    for (mut transform, pos) in &mut query {
        transform.translation = (*pos).into();
    }
}
