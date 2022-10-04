use bevy::prelude::*;
use rand::Rng;

use super::group_colors::HIGH_CONTRAST_COLORS;
use crate::consts::{ITEM_RADIUS, ITEM_SUBDIVISIONS};
use crate::dataset::{Dataset, DatasetHandle};
use crate::{BoardPosition, IcoBoard};

#[derive(Component)]
pub struct Item {
    pub data: [f32; 2],
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
                            if board.get_cell(&pos).food.is_none() {
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
                            .insert(pos)
                            .id();

                        board.get_cell_mut(&pos).food = Some(id);
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn item_position_update(
    mut query: Query<
        (&mut Transform, &mut Visibility, &BoardPosition),
        (Changed<BoardPosition>, With<Item>),
    >,
    board: Res<IcoBoard>,
) {
    for (mut transform, mut visibility, pos) in &mut query {
        transform.translation = board.world_position(pos).into();
        visibility.is_visible = true;
    }
}

pub fn item_pickup_update(
    removals: RemovedComponents<BoardPosition>,
    mut query: Query<&mut Visibility>,
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
