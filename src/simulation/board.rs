use crate::simulation::board_position::BoardPosition;
use crate::WINDOW_SIZE;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell;

type Matrix<T> = Vec<Vec<T>>;

pub const BOARD_SIZE: usize = (WINDOW_SIZE / CELL_SIZE) as usize;
pub const CELL_BORDER: f32 = 1.;
pub const CELL_SIZE: f32 = 10.;
pub const CELL_PAINT: f32 = CELL_SIZE - 2. * CELL_BORDER;

#[derive(Debug, Default)]
pub struct Board {
    width: usize,
    height: usize,
    values: Matrix<Cell>,
}

impl Deref for Board {
    type Target = Matrix<Cell>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: vec![vec![Cell::default(); height]; width],
        }
    }
}

pub fn board_setup(mut commands: Commands, board: Res<Board>) {
    for i in 0..board.width {
        for j in 0..board.height {
            let board_position = BoardPosition::new(i as u32, j as u32);
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(CELL_PAINT, CELL_PAINT)),
                    ..default()
                },
                transform: board_position.into(),
                ..default()
            });
        }
    }
}
