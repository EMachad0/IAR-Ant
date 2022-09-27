use bevy::prelude::*;
use rand::Rng;

use crate::consts::{ANT_COUNT, ANT_HEIGHT, ANT_RADIUS};
use crate::simulation::board::BoardPosition;
use crate::{IcoBoard, SimulationStatus};

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Ant {
    pub item: Option<Entity>,
}

pub fn ant_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board: Res<IcoBoard>,
) {
    let ant_mesh = meshes.add(
        shape::Capsule {
            radius: ANT_RADIUS,
            depth: ANT_HEIGHT,
            latitudes: 8,
            longitudes: 16,
            ..default()
        }
        .into(),
    );
    let ant_material = materials.add(Color::BLACK.into());
    for _ in 0..ANT_COUNT {
        let pos = board.new_random_position();

        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: ant_mesh.clone(),
                material: ant_material.clone(),
                transform: Transform {
                    translation: board.world_position(&pos).into(),
                    rotation: Quat::from_rotation_arc(
                        Vec3::Y,
                        Vec3::from(board.world_position(&pos)).normalize(),
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Ant::default())
            .insert(pos);
    }
}

pub fn ant_move(
    status: Res<SimulationStatus>,
    mut query: Query<(&Ant, &mut BoardPosition)>,
    board: Res<IcoBoard>,
) {
    for (ant, mut pos) in &mut query {
        if status.ending && ant.item.is_none() {
            continue;
        }
        *pos = board.get_random_adjacent(&pos);
    }
}

pub fn ant_pickup_drop(
    status: Res<SimulationStatus>,
    mut commands: Commands,
    mut query: Query<(&BoardPosition, &mut Ant)>,
    mut board: ResMut<IcoBoard>,
) {
    let mut rng = rand::thread_rng();
    for (pos, mut ant) in &mut query {
        let (empty_cells, food_cells) = {
            let mut empty_cells = 0;
            let mut food_cells = 0;
            for lookup_pos in board.get_all_adjacent(pos) {
                match board.get_cell(&lookup_pos).food {
                    None => empty_cells += 1,
                    Some(_) => food_cells += 1,
                }
            }
            (empty_cells as f64, food_cells as f64)
        };
        let ratio = food_cells / (food_cells + empty_cells);
        let threshold = 100. / 80.;
        let prob = (ratio * threshold).min(1.);

        match (board.get_cell(pos).food, ant.item) {
            (Some(item), None) => {
                if !status.ending && rng.gen_bool(1. - prob) {
                    commands.entity(item).remove::<BoardPosition>();
                    ant.item = board.get_cell_mut(pos).food.take();
                }
            }
            (None, Some(item)) => {
                if rng.gen_bool(prob) {
                    commands.entity(item).insert(*pos);
                    board.get_cell_mut(pos).food = ant.item.take();
                }
            }
            (_, _) => {}
        }
    }
}

pub fn ant_texture_update(
    mut query: Query<(&mut Handle<StandardMaterial>, &Ant), Changed<Ant>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut material, ant) in &mut query {
        match ant.item {
            Some(_) => *material = materials.add(Color::CRIMSON.into()),
            None => *material = materials.add(Color::BLACK.into()),
        }
    }
}
