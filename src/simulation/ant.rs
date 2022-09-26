use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::consts::{ANT_COUNT, BOARD_HEIGHT, BOARD_WIDTH, CELL_PAINT, VIEW_RADIUS};
use crate::simulation::board::{BoardEntity, BoardPosition};
use crate::{Board, SimulationStatus};

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Ant {
    pub food: Option<Entity>,
}

pub fn ant_spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    if ANT_COUNT > BOARD_WIDTH * BOARD_HEIGHT {
        let error_message = "More ants than board cells";
        error!("{error_message}");
        panic!("{error_message}");
    }

    let mut rng = rand::thread_rng();
    let range_x = Uniform::from(0..BOARD_WIDTH as i32);
    let range_y = Uniform::from(0..BOARD_HEIGHT as i32);
    for _ in 0..ANT_COUNT {
        let pos = loop {
            let x = rng.sample(range_x);
            let y = rng.sample(range_y);
            let pos = BoardPosition::new(x, y).unwrap();
            if board.get_cell(&pos).ant.is_none() {
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
            .insert(Ant::default())
            .insert(BoardEntity)
            .insert(pos)
            .id();

        board.get_cell_mut(&pos).ant = Some(id);
    }
}

pub fn ant_move(
    status: Res<SimulationStatus>,
    mut query: Query<(Entity, &Ant, &mut BoardPosition)>,
    mut board: ResMut<Board>,
) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(-1..=1);
    for (id, ant, mut pos) in &mut query {
        if status.ending && ant.food.is_none() {
            continue;
        }

        board.get_cell_mut(&*pos).ant = None;

        let new_pos = loop {
            let dx = rng.sample(range);
            let dy = rng.sample(range);
            if let Ok(new_pos) = pos.add(dx, dy) {
                if board.get_cell(&new_pos).ant.is_none() {
                    break new_pos;
                }
            }
        };

        *pos = new_pos;
        board.get_cell_mut(&new_pos).ant = Some(id);
    }
}

pub fn ant_pickup_drop(
    status: Res<SimulationStatus>,
    mut commands: Commands,
    mut query: Query<(&BoardPosition, &mut Ant)>,
    mut board: ResMut<Board>,
) {
    let mut rng = rand::thread_rng();
    for (pos, mut ant) in &mut query {
        let (empty_cells, food_cells) = {
            let mut empty_cells = 0;
            let mut food_cells = 0;
            for dx in -VIEW_RADIUS..=VIEW_RADIUS {
                for dy in -VIEW_RADIUS..=VIEW_RADIUS {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Ok(lookup_pos) = pos.add(dx, dy) {
                        match board.get_cell(&lookup_pos).food {
                            None => empty_cells += 1,
                            Some(_) => food_cells += 1,
                        }
                    }
                }
            }
            (empty_cells as f64, food_cells as f64)
        };
        let ratio = food_cells / (food_cells + empty_cells);
        let threshold = 100. / 80.;
        let prob = (ratio * threshold).min(1.);

        match (board.get_cell(pos).food, ant.food) {
            (Some(food), None) => {
                if !status.ending && rng.gen_bool(1. - prob) {
                    commands.entity(food).remove::<BoardPosition>();
                    ant.food = board.get_cell_mut(pos).food.take();
                }
            }
            (None, Some(food)) => {
                if rng.gen_bool(prob) {
                    commands.entity(food).insert(*pos);
                    board.get_cell_mut(pos).food = ant.food.take();
                }
            }
            (_, _) => {}
        }
    }
}

pub fn ant_texture_update(
    mut query: Query<(&mut Handle<Image>, &Ant), Changed<Ant>>,
    asset_server: Res<AssetServer>,
) {
    for (mut image, ant) in &mut query {
        match ant.food {
            None => *image = asset_server.load("img/empty_ant.png"),
            Some(_) => *image = asset_server.load("img/carry_ant.png"),
        }
    }
}
