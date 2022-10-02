use bevy::prelude::*;

use crate::consts::{ITEM_COUNT, ITEM_RADIUS, ITEM_SUBDIVISIONS};
use crate::{BoardPosition, IcoBoard};

#[derive(Component)]
pub struct Item;

pub fn item_spawn(
    mut commands: Commands,
    mut board: ResMut<IcoBoard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if ITEM_COUNT > board.size() {
        let error_message = format!(
            "More food ({}) than board cells ({})",
            ITEM_COUNT,
            board.size()
        );
        error!("{error_message}");
        panic!("{error_message}");
    }

    let item_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: ITEM_RADIUS,
        subdivisions: ITEM_SUBDIVISIONS,
    }));
    let item_material = materials.add(Color::RED.into());

    for _ in 0..ITEM_COUNT {
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
            .insert(Item)
            .insert(pos)
            .id();

        board.get_cell_mut(&pos).food = Some(id);
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
