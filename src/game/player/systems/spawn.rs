use bevy::prelude::*;

use crate::game::player::components::Player;

pub fn spawn_player(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, -5.0, 1.71)
        .looking_at(Vec3::new(0.0, 0.0, 1.71), Vec3::Z);

    commands.spawn((
        // Player is simply a camera for now!
        Camera3dBundle { transform, ..default() },
        Player::default(),
    ));
}

