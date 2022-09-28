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
    mut query: Query<(&mut Transform, &mut Visibility, &BoardPosition), Changed<BoardPosition>>,
    board: Res<IcoBoard>,
) {
    for (mut transform, mut visibility, pos) in &mut query {
        transform.translation = board.world_position(pos).into();
        transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, Vec3::from(board.world_position(&pos)).normalize());
        transform.scale = Vec3::ONE;
        visibility.is_visible = true;
    }
}

pub fn update_removed_board_position(
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
