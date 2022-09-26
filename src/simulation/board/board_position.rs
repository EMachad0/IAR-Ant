use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::Rng;

use crate::consts::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};
use crate::simulation::board::BoardEntity;

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoardPosition {
    x: usize,
    y: usize,
}

impl BoardPosition {
    pub fn new(x: i32, y: i32) -> Self {
        let width = BOARD_WIDTH as i32;
        let height = BOARD_HEIGHT as i32;
        let x = (x % width + width) % width;
        let y = (y % height + height) % height;
        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    pub fn to_world_position(&self) -> (f32, f32) {
        let x_off = (self.x as f32) * CELL_SIZE;
        let y_off = (self.y as f32) * CELL_SIZE;
        let x = x_off + CELL_SIZE / 2.;
        let y = y_off + CELL_SIZE / 2.;
        (x, y)
    }

    pub fn add(&self, dx: i32, dy: i32) -> Self {
        let (x, y) = (self.x as i32 + dx, self.y as i32 + dy);
        Self::new(x, y)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..BOARD_WIDTH as i32);
        let y = rng.gen_range(0..BOARD_HEIGHT as i32);
        BoardPosition::new(x, y)
    }

    pub fn get_all_adjacent(&self, radius: i32) -> Vec<Self> {
        let mut adj = Vec::new();
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let lookup_pos = self.add(dx, dy);
                adj.push(lookup_pos);
            }
        }
        adj
    }

    pub fn get_random_adjacent(&self) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(-1..=1);
        let dx = rng.sample(range);
        let dy = rng.sample(range);
        self.add(dx, dy)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
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
