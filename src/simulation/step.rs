use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct Actor;

pub fn setup(mut commands: Commands) {
    // Square
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        })
        .insert(Actor);
}

pub fn step(mut query: Query<&mut Transform, With<Actor>>, windows: Res<Windows>) {
    let window = windows.get_primary().expect("Could not find a window");
    for mut transform in query.iter_mut() {
        transform.translation += vec3(1.0, 1.0, 0.0);
        transform.translation.x = (transform.translation.x + window.width()) % window.width();
        transform.translation.y = (transform.translation.y + window.height()) % window.height();
    }
}
