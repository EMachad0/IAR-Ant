use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::consts::{BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT, FOOD_COUNT};
use crate::simulation::board::{BoardEntity, BoardPosition};
use crate::Board;

#[derive(Component)]
pub struct Food;

pub fn food_spawn(mut commands: Commands, mut board: ResMut<Board>) {
    if FOOD_COUNT > BOARD_WIDTH * BOARD_HEIGHT {
        let error_message = format!(
            "More food ({}) than board cells ({})",
            FOOD_COUNT,
            BOARD_WIDTH * BOARD_HEIGHT
        );
        error!("{error_message}");
        panic!("{error_message}");
    }

    let mut rng = rand::thread_rng();
    let range_x = Uniform::from(0..BOARD_WIDTH as i32);
    let range_y = Uniform::from(0..BOARD_HEIGHT as i32);
    for _ in 0..FOOD_COUNT {
        let pos = loop {
            let x = rng.sample(range_x);
            let y = rng.sample(range_y);
            let pos = BoardPosition::new(x, y).unwrap();
            if board.get_cell(&pos).food.is_none() {
                break pos;
            }
        };

        let id = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(CELL_PAINT, CELL_PAINT)),
                    ..default()
                },
                transform: pos.into(),
                ..default()
            })
            .insert(Food)
            .insert(BoardEntity)
            .insert(pos)
            .id();

        board.get_cell_mut(&pos).food = Some(id);
    }
}
