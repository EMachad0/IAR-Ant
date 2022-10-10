use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use rand::Rng;

use crate::consts::{ANT_COUNT, ANT_HEIGHT, ANT_RADIUS, BOARD_RADIUS};
use crate::simulation::ant::{drop_probability, pickup_probability};
use crate::simulation::board::BoardPosition;
use crate::simulation::item;
use crate::simulation::item::Item;
use crate::{IcoBoard, SimulationStatus};

const TRANSLATION_MULTIPLIER: f32 = 1. + (ANT_HEIGHT + 2. * ANT_RADIUS) / (2. * BOARD_RADIUS);

pub const ANT_MATERIAL: HandleUntyped =
    HandleUntyped::weak_from_u64(StandardMaterial::TYPE_UUID, 536721377579319359);

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
    materials.set_untracked(ANT_MATERIAL, Color::BLACK.into());

    let ant_mesh = meshes.add(
        shape::Capsule {
            radius: ANT_RADIUS,
            depth: ANT_HEIGHT,
            latitudes: 4,
            longitudes: 8,
            ..default()
        }
        .into(),
    );

    for _ in 0..ANT_COUNT {
        let pos = board.new_random_position();

        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: ant_mesh.clone(),
                material: ANT_MATERIAL.typed_weak(),
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
        *pos = *board.get_random_adjacent(&pos);
    }
}

pub fn ant_pickup_drop(
    status: Res<SimulationStatus>,
    mut board: ResMut<IcoBoard>,
    mut query: Query<(&BoardPosition, &mut Ant)>,
    item_query: Query<&Item>,
    mut visibility_query: Query<&mut Visibility, With<Item>>,
) {
    let mut rng = rand::thread_rng();
    for (pos, mut ant) in &mut query {
        match (board.get_cell(pos).item, ant.item) {
            (Some(entity), None) => {
                if status.ending {
                    continue;
                }

                let similarity = item::compute_similarity(entity, pos, &board, &item_query);

                if rng.gen_bool(pickup_probability(similarity)) {
                    let mut visibility = visibility_query.get_mut(entity).unwrap();
                    visibility.is_visible = false;
                    ant.item = board.get_cell_mut(pos).item.take();
                }
            }
            (None, Some(entity)) => {
                let similarity = item::compute_similarity(entity, pos, &board, &item_query);

                if rng.gen_bool(drop_probability(similarity)) {
                    let mut visibility = visibility_query.get_mut(entity).unwrap();
                    visibility.is_visible = true;
                    board.get_cell_mut(pos).item = ant.item.take();
                }
            }
            (_, _) => {}
        }
    }
}

pub fn ant_texture_update(
    mut query: Query<(&mut Handle<StandardMaterial>, &Ant), (Changed<Ant>, Without<Item>)>,
    item_query: Query<&Handle<StandardMaterial>, With<Item>>,
) {
    for (mut material, ant) in &mut query {
        match ant.item {
            Some(entity) => *material = item_query.get(entity).unwrap().clone(),
            None => *material = ANT_MATERIAL.typed_weak(),
        }
    }
}

pub fn ant_position_update(
    mut query: Query<(&mut Transform, &BoardPosition), (Changed<BoardPosition>, With<Ant>)>,
    board: Res<IcoBoard>,
) {
    for (mut transform, pos) in &mut query {
        let mut translation: Vec3 = board.world_position(pos).into();
        translation *= TRANSLATION_MULTIPLIER;
        transform.translation = translation;
        transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, Vec3::from(board.world_position(&pos)).normalize());
    }
}

pub fn ant_carried_item_position_update(
    mut items: Query<&mut BoardPosition, With<Item>>,
    ants: Query<(&BoardPosition, &Ant), (Changed<BoardPosition>, Without<Item>)>,
) {
    for (pos, ant) in &ants {
        if let Some(entity) = ant.item {
            *items.get_mut(entity).unwrap() = *pos;
        }
    }
}
