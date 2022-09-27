use bevy::prelude::*;

use crate::consts::{FOOD_RADIUS, ITEM_COUNT};
use crate::IcoBoard;

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
        radius: FOOD_RADIUS,
        subdivisions: 1,
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
