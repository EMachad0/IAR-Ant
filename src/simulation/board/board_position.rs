use crate::IcoBoard;
use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Component, Reflect, Deref)]
#[reflect(Component)]
pub struct BoardPosition(usize);

impl BoardPosition {
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub fn idx(&self) -> usize {
        self.0
    }
}

pub fn update_board_position(
    mut query: Query<(&mut Transform, &BoardPosition), Changed<BoardPosition>>,
    board: Res<IcoBoard>,
) {
    for (mut transform, pos) in &mut query {
        transform.translation = board.world_position(pos).into();
        transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, Vec3::from(board.world_position(&pos)).normalize());
        transform.scale = Vec3::ONE;
    }
}
