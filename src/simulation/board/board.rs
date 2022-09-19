use bevy::prelude::*;
use std::ops::{Index, IndexMut};
use bevy_inspector_egui::Inspectable;

use super::{BoardPosition, Cell};
use crate::consts::{BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT};

#[derive(Debug, Default, Clone, Deref, DerefMut, Reflect, Inspectable)]
#[reflect(Resource)]
pub struct Board(Vec<Cell>);

impl Board {
    pub fn new() -> Self {
        let values = vec![Cell::default(); BOARD_WIDTH * BOARD_HEIGHT];
        Self(values)
    }

    pub fn get_cell(&self, pos: &BoardPosition) -> &Cell {
        &self[pos.x()][pos.y()]
    }

    pub fn get_cell_mut(&mut self, pos: &BoardPosition) -> &mut Cell {
        &mut self[pos.x()][pos.y()]
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

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let begin = index * BOARD_WIDTH;
        let end = begin + BOARD_WIDTH;
        &mut self.0[begin..end]
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
