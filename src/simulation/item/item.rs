use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent};
use rand::Rng;

use super::group_colors::HIGH_CONTRAST_COLORS;
use crate::consts::{ALPHA, ITEM_RADIUS, ITEM_SUBDIVISIONS};
use crate::dataset::{Dataset, DatasetHandle};
use crate::{BoardPosition, IcoBoard};

#[derive(Debug, Component)]
pub struct Item {
    pub data: [f32; 2],
}

impl Item {
    pub fn dis(&self, other: &Self) -> f64 {
        let [x1, y1] = self.data;
        let [x2, y2] = other.data;
        ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt() as f64
    }
}

pub fn item_spawn_on_dataset_load(
    mut ev_asset: EventReader<AssetEvent<Dataset>>,
    assets: ResMut<Assets<Dataset>>,
    dataset_handle: Res<DatasetHandle>,
    mut commands: Commands,
    mut board: ResMut<IcoBoard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle != **dataset_handle {
                    continue;
                }

                let dataset = assets.get(handle).unwrap();

                if dataset.data.len() > 22 {
                    warn!("More point groups than colors using random colors!!")
                }

                let item_count: usize = dataset.data.iter().map(|(_, v)| v.len()).sum();
                if item_count > board.size() {
                    let error_message = format!(
                        "More food ({}) than board cells ({})",
                        item_count,
                        board.size()
                    );
                    error!("{error_message}");
                    panic!("{error_message}");
                }

                let item_mesh = meshes.add(Mesh::from(shape::Icosphere {
                    radius: ITEM_RADIUS,
                    subdivisions: ITEM_SUBDIVISIONS,
                }));

                let mut color_iterator = HIGH_CONTRAST_COLORS.iter();
                for (_, v) in &dataset.data {
                    let item_material = {
                        let (r, g, b) = color_iterator.next().map(|f| *f).unwrap_or(rng.gen());
                        let group_color = Color::rgb_u8(r, g, b);
                        materials.add(group_color.into())
                    };

                    for data in v {
                        let pos = loop {
                            let pos = board.new_random_position();
                            if board.get_cell(&pos).item.is_none() {
                                break pos;
                            }
                        };

                        let id = commands
                            .spawn()
                            .insert_bundle(PbrBundle {
                                mesh: item_mesh.clone(),
                                material: item_material.clone(),
                                transform: Transform {
                                    translation: Vec3::from(board.world_position(&pos)),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Item { data: *data })
                            .insert_bundle(PickableBundle::default())
                            .insert(pos)
                            .id();

                        board.get_cell_mut(&pos).item = Some(id);
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn compute_similarity(
    entity: Entity,
    pos: &BoardPosition,
    board: &IcoBoard,
    item_query: &Query<&Item>,
) -> f64 {
    let item = item_query.get(entity).expect("Expect entity to be an Item");
    let s = board.get_all_adjacent(pos).len() as f64;
    let sum: f64 = board
        .get_all_adjacent(pos)
        .iter()
        .filter_map(|other_pos| board.get_cell(other_pos).item)
        .map(|entity| item_query.get(entity).unwrap())
        .map(|other| 1. - item.dis(other) / ALPHA)
        .sum();
    (sum / s).max(0.0)
}

pub fn item_position_update(
    mut query: Query<(&mut Transform, &BoardPosition), (Changed<BoardPosition>, With<Item>)>,
    board: Res<IcoBoard>,
) {
    for (mut transform, pos) in &mut query {
        transform.translation = board.world_position(pos).into();
    }
}

pub fn print_on_pick(mut events: EventReader<PickingEvent>, query: Query<&Item>) {
    for ev in events.iter() {
        match ev {
            PickingEvent::Clicked(entity) => {
                let item = query.get(*entity).unwrap();
                info!("{:?}", item);
            }
            _ => (),
        }
    }
}
