use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::consts::{ANT_COUNT, BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT};
use crate::simulation::board_position::BoardPosition;

#[derive(Component)]
pub struct Ant;

pub fn ant_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let range_x = Uniform::from(0..BOARD_WIDTH as i32);
    let range_y = Uniform::from(0..BOARD_HEIGHT as i32);
    for _ in 0..ANT_COUNT {
        let x = rng.sample(range_x);
        let y = rng.sample(range_y);
        let pos = BoardPosition::new(x, y).unwrap();
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("img/empty_ant.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(CELL_PAINT, CELL_PAINT)),
                    ..default()
                },
                transform: pos.into(),
                ..default()
            })
            .insert(Ant)
            .insert(pos);
    }
}

pub fn ant_move(mut query: Query<&mut BoardPosition, With<Ant>>) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(-1..=1);
    for mut pos in &mut query {
        loop {
            let dx = rng.sample(range);
            let dy = rng.sample(range);
            if let Ok(new_pos) = pos.add(dx, dy) {
                *pos = new_pos;
                break;
            }
        }
    }
}
