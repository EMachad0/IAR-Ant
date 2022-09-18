use crate::simulation::board::CELL_PAINT;
use crate::simulation::board_position::BoardPosition;
use crate::BOARD_SIZE;
pub use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

const ANT_COUNT: u32 = 500;

#[derive(Component)]
pub struct Ant;

pub fn ant_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..BOARD_SIZE as u32);
    for _ in 0..ANT_COUNT {
        let x = rng.sample(range);
        let y = rng.sample(range);
        let pos = BoardPosition::new(x, y);
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
        let dx = rng.sample(range);
        let dy = rng.sample(range);
        pos.add(dx, dy);
    }
}
