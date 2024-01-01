use bevy::prelude::*;

use crate::game::player::components::Player;

pub fn spawn_player(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, -4.5, 1.71).looking_at(Vec3::ZERO, Vec3::Z);
    //transform.rotation = Quat::from_axis_angle(Vec3::Y, 0.0);

    commands.spawn((
        // Player is simply a camera for now!
        Camera3dBundle { transform, ..default() },
        Player::default(),
    ));
}

