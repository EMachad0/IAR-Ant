use crate::Board;
use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::consts::{ANT_COUNT, BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT};
use crate::simulation::board::BoardPosition;

#[derive(Component)]
pub struct Ant;

pub fn ant_spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    if ANT_COUNT > BOARD_WIDTH * BOARD_HEIGHT {
        error!("More ants than board cells");
        panic!("More ants than board cells");
    }

    let mut rng = rand::thread_rng();
    let range_x = Uniform::from(0..BOARD_WIDTH as i32);
    let range_y = Uniform::from(0..BOARD_HEIGHT as i32);
    for _ in 0..ANT_COUNT {
        let pos = loop {
            let x = rng.sample(range_x);
            let y = rng.sample(range_y);
            let pos = BoardPosition::new(x, y).unwrap();
            if board.get_cell(pos).ant.is_none() {
                break pos;
            }
        };

        let id = commands
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
            .insert(pos)
            .id();

        board.get_cell_mut(pos).ant = Some(id);
    }
}

pub fn ant_move(
    mut query: Query<(Entity, &mut BoardPosition), With<Ant>>,
    mut board: ResMut<Board>,
) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(-1..=1);
    for (id, mut pos) in &mut query {
        board.get_cell_mut(*pos).ant = None;

        let new_pos = loop {
            let dx = rng.sample(range);
            let dy = rng.sample(range);
            if let Ok(new_pos) = pos.add(dx, dy) {
                if board.get_cell(new_pos).ant.is_none() {
                    break new_pos;
                }
            }
        };

        *pos = new_pos;
        board.get_cell_mut(new_pos).ant = Some(id);
    }
}
