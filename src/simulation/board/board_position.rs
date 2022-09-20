use bevy::prelude::*;

use crate::consts::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};
use crate::simulation::board::BoardEntity;

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoardPosition {
    x: usize,
    y: usize,
}

impl BoardPosition {
    pub fn new(x: i32, y: i32) -> Result<Self, InvalidBoardPositionError> {
        if Self::is_valid_position(x, y) {
            Ok(Self {
                x: x as usize,
                y: y as usize,
            })
        } else {
            Err(InvalidBoardPositionError { x, y })
        }
    }

    pub fn to_world_position(&self) -> (f32, f32) {
        let x_off = (self.x as f32) * CELL_SIZE;
        let y_off = (self.y as f32) * CELL_SIZE;
        let x = x_off + CELL_SIZE / 2.;
        let y = y_off + CELL_SIZE / 2.;
        (x, y)
    }

    pub fn add(&self, dx: i32, dy: i32) -> Result<Self, InvalidBoardPositionError> {
        let (x, y) = (self.x as i32 + dx, self.y as i32 + dy);
        Self::new(x, y)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn is_valid_position(x: i32, y: i32) -> bool {
        0 <= x && x < BOARD_WIDTH as i32 && 0 <= y && y < BOARD_HEIGHT as i32
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
    mut query: Query<(&mut Transform, &mut Visibility, &BoardPosition), Changed<BoardPosition>>,
) {
    for (mut transform, mut visibility, pos) in &mut query {
        visibility.is_visible = true;
        transform.translation = (*pos).into();
    }
}

pub fn update_removed_board_position(
    removals: RemovedComponents<BoardPosition>,
    mut query: Query<&mut Visibility, With<BoardEntity>>,
) {
    for entity in removals.iter() {
        let mut visibility = query.get_mut(entity).unwrap_or_else(|_| {
            let error_message = "Could not find BoardEntity with removed BoardPosition";
            error!("{error_message}");
            panic!("{error_message}");
        });
        visibility.is_visible = false;
    }
}

#[derive(Debug, Clone)]
pub struct InvalidBoardPositionError {
    x: i32,
    y: i32,
}

impl std::fmt::Display for InvalidBoardPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attempted to create an invalid position")
    }
}
