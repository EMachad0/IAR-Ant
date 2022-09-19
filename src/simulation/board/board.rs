use bevy::prelude::*;
use std::ops::Index;

use crate::consts::{BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT};
use crate::simulation::board::BoardPosition;

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell;

type Matrix<T> = Box<[T]>;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Board(Matrix<Cell>);

impl Board {
    pub fn new() -> Self {
        let values = vec![Cell::default(); BOARD_WIDTH * BOARD_HEIGHT].into_boxed_slice();
        Self(values)
    }
}

impl Index<usize> for Board {
    type Output = [Cell];

    fn index(&self, index: usize) -> &Self::Output {
        let begin = index * BOARD_WIDTH;
        let end = begin + BOARD_WIDTH;
        &self.0[begin..end]
    }
}

pub fn board_setup(mut commands: Commands) {
    for i in 0..BOARD_WIDTH {
        for j in 0..BOARD_HEIGHT {
            let board_position = BoardPosition::new(i as i32, j as i32).unwrap();
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
